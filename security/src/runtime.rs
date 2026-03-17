//! WASM tool runtime for managing compiled components in OfficeClaw.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use wasmtime::{Config, Engine, OptLevel};

use crate::error::WasmError;
use crate::limits::{FuelConfig, ResourceLimits};

/// Default epoch tick interval for timeout mechanisms.
pub const EPOCH_TICK_INTERVAL: Duration = Duration::from_millis(500);

/// Enable wasmtime's persistent compilation cache.
pub fn enable_compilation_cache(
    wasmtime_config: &mut Config,
    _label: &str,
    explicit_dir: Option<&Path>,
) -> anyhow::Result<()> {
    if let Some(dir) = explicit_dir {
        std::fs::create_dir_all(dir)?;
        let toml_path = dir.join("wasmtime-cache.toml");
        let escaped = dir.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"");
        let toml_content = format!("[cache]\nenabled = true\ndirectory = \"{}\"\n", escaped);
        std::fs::write(&toml_path, toml_content)?;
        wasmtime_config.cache_config_load(&toml_path)?;
    } else {
        wasmtime_config.cache_config_load_default()?;
    }
    Ok(())
}

/// Configuration for the WASM runtime.
#[derive(Debug, Clone)]
pub struct WasmRuntimeConfig {
    pub default_limits: ResourceLimits,
    pub fuel_config: FuelConfig,
    pub cache_compiled: bool,
    pub cache_dir: Option<PathBuf>,
    pub optimization_level: OptLevel,
}

impl Default for WasmRuntimeConfig {
    fn default() -> Self {
        Self {
            default_limits: ResourceLimits::default(),
            fuel_config: FuelConfig::default(),
            cache_compiled: true,
            cache_dir: None,
            optimization_level: OptLevel::Speed,
        }
    }
}

impl WasmRuntimeConfig {
    pub fn for_testing() -> Self {
        Self {
            default_limits: ResourceLimits::default()
                .with_memory(1024 * 1024)
                .with_fuel(100_000)
                .with_timeout(Duration::from_secs(5)),
            fuel_config: FuelConfig::with_limit(100_000),
            cache_compiled: false,
            cache_dir: None,
            optimization_level: OptLevel::None,
        }
    }
}

pub struct PreparedModule {
    pub name: String,
    pub description: String,
    pub schema: serde_json::Value,
    component: wasmtime::component::Component,
    pub limits: ResourceLimits,
}

impl PreparedModule {
    pub fn component(&self) -> &wasmtime::component::Component {
        &self.component
    }
}

impl std::fmt::Debug for PreparedModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedModule")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("limits", &self.limits)
            .finish()
    }
}

pub struct WasmToolRuntime {
    engine: Engine,
    config: WasmRuntimeConfig,
    modules: RwLock<HashMap<String, Arc<PreparedModule>>>,
}

impl WasmToolRuntime {
    pub fn new(config: WasmRuntimeConfig) -> Result<Self, WasmError> {
        let mut wasmtime_config = Config::new();

        if config.fuel_config.enabled {
            wasmtime_config.consume_fuel(true);
        }

        wasmtime_config.epoch_interruption(true);
        wasmtime_config.wasm_component_model(true);
        wasmtime_config.wasm_threads(false);
        wasmtime_config.cranelift_opt_level(config.optimization_level);
        wasmtime_config.debug_info(false);

        if let Err(e) = enable_compilation_cache(&mut wasmtime_config, "tools", config.cache_dir.as_deref()) {
            tracing::warn!("Failed to enable wasmtime compilation cache: {}", e);
        }

        let engine = Engine::new(&wasmtime_config).map_err(|e| {
            WasmError::EngineCreationFailed(format!("Failed to create Wasmtime engine: {}", e))
        })?;

        let ticker_engine = engine.clone();
        std::thread::Builder::new()
            .name("wasm-epoch-ticker".into())
            .spawn(move || {
                loop {
                    std::thread::sleep(EPOCH_TICK_INTERVAL);
                    ticker_engine.increment_epoch();
                }
            })
            .map_err(|e| {
                WasmError::EngineCreationFailed(format!("Failed to spawn epoch ticker thread: {}", e))
            })?;

        Ok(Self {
            engine,
            config,
            modules: RwLock::new(HashMap::new()),
        })
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn config(&self) -> &WasmRuntimeConfig {
        &self.config
    }

    pub async fn prepare(
        &self,
        name: &str,
        wasm_bytes: &[u8],
        limits: Option<ResourceLimits>,
    ) -> Result<Arc<PreparedModule>, WasmError> {
        if let Some(module) = self.modules.read().await.get(name) {
            return Ok(Arc::clone(module));
        }

        let name = name.to_string();
        let wasm_bytes = wasm_bytes.to_vec();
        let engine = self.engine.clone();
        let default_limits = self.config.default_limits.clone();

        let prepared = tokio::task::spawn_blocking(move || {
            let component = wasmtime::component::Component::new(&engine, &wasm_bytes)
                .map_err(|e| WasmError::CompilationFailed(e.to_string()))?;

            let effective_limits = limits.clone().unwrap_or(default_limits.clone());
            let (description, schema) = crate::wrapper::extract_wasm_metadata(
                &engine,
                &component,
                &effective_limits,
            ).unwrap_or_else(|_| (
                "WASM tool".to_string(),
                serde_json::json!({"type": "object"})
            ));

            Ok::<_, WasmError>(PreparedModule {
                name,
                description,
                schema,
                component,
                limits: limits.unwrap_or(default_limits),
            })
        })
        .await
        .map_err(|e| WasmError::ExecutionPanicked(format!("Preparation task panicked: {}", e)))??;

        let prepared = Arc::new(prepared);

        if self.config.cache_compiled {
            self.modules.write().await.insert(prepared.name.clone(), Arc::clone(&prepared));
        }

        Ok(prepared)
    }

    pub async fn get(&self, name: &str) -> Option<Arc<PreparedModule>> {
        self.modules.read().await.get(name).cloned()
    }

    pub async fn list(&self) -> Vec<String> {
        self.modules.read().await.keys().cloned().collect()
    }

    pub async fn clear(&self) {
        self.modules.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_config_default() {
        let config = WasmRuntimeConfig::default();
        assert!(config.cache_compiled);
        assert!(config.fuel_config.enabled);
    }

    #[test]
    fn test_runtime_creation() {
        let config = WasmRuntimeConfig::for_testing();
        let runtime = WasmToolRuntime::new(config).unwrap();
        assert!(runtime.config().fuel_config.enabled);
    }

    #[tokio::test]
    async fn test_module_cache_operations() {
        let config = WasmRuntimeConfig::for_testing();
        let runtime = WasmToolRuntime::new(config).unwrap();
        assert!(runtime.list().await.is_empty());
    }
}
