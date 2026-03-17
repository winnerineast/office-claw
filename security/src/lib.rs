use napi_derive::napi;
use std::sync::Arc;
use std::sync::OnceLock;

pub mod capabilities;
pub mod error;
pub mod limits;
pub mod loader;
pub mod runtime;
pub mod wrapper;

pub use error::{WasmError, ToolError};
pub use limits::ResourceLimits;
pub use runtime::{WasmRuntimeConfig, WasmToolRuntime};

// Global runtime instance for NAPI calls (Lazy Init)
static GLOBAL_RUNTIME: OnceLock<Arc<WasmToolRuntime>> = OnceLock::new();

#[napi]
pub async fn init_runtime() -> Result<(), napi::Error> {
    let config = WasmRuntimeConfig::default();
    let runtime = WasmToolRuntime::new(config).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let _ = GLOBAL_RUNTIME.get_or_init(|| Arc::new(runtime));
    Ok(())
}

#[napi]
pub async fn execute_skill(name: String, wasm_bytes: napi::bindgen_prelude::Uint8Array, params_json: String) -> Result<String, napi::Error> {
    let runtime = GLOBAL_RUNTIME.get()
        .ok_or_else(|| napi::Error::from_reason("Runtime not initialized. Call init_runtime() first."))?;
    
    // Convert napi::Uint8Array to &[u8]
    let prepared = runtime.prepare(&name, &wasm_bytes, None).await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let wrapper = wrapper::WasmToolWrapper::new(
        Arc::clone(runtime),
        prepared,
        capabilities::Capabilities::default(),
    );

    let params: serde_json::Value = serde_json::from_str(&params_json)
        .map_err(|e| napi::Error::from_reason(format!("Invalid JSON: {}", e)))?;

    let ctx = JobContext {
        user_id: "default_user".to_string(),
        session_id: "default_session".to_string(),
    };

    let output = crate::Tool::execute(&wrapper, params, &ctx).await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(serde_json::to_string(&output.result).unwrap_or_default())
}

/// Simplified JobContext for OfficeClaw.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JobContext {
    pub user_id: String,
    pub session_id: String,
}

/// Simplified Tool trait for OfficeClaw.
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value, ctx: &JobContext) -> Result<ToolOutput, ToolError>;
}

/// Simplified ToolOutput for OfficeClaw.
#[derive(Debug, Clone)]
pub struct ToolOutput {
    pub result: serde_json::Value,
    pub duration: std::time::Duration,
}

impl ToolOutput {
    pub fn success(result: serde_json::Value, duration: std::time::Duration) -> Self {
        Self { result, duration }
    }
}

/// Placeholder for LeakDetector.
pub struct LeakDetector;
impl LeakDetector {
    pub fn new() -> Self { Self }
    pub fn scan_and_clean(&self, input: &str) -> Result<String, String> {
        Ok(input.to_string())
    }
}

/// Placeholder for SecretsStore.
pub trait SecretsStore: Send + Sync {
    fn get_secret(&self, name: &str) -> Option<String>;
}
