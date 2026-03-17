//! WASM tool wrapper implementing the Tool trait for OfficeClaw.

use std::sync::Arc;
use std::time::Instant;

use wasmtime::Store;
use wasmtime::component::Linker;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::error::{WasmError, ToolError};
use crate::capabilities::Capabilities;
use crate::limits::{ResourceLimits, WasmResourceLimiter};
use crate::runtime::{EPOCH_TICK_INTERVAL, PreparedModule, WasmToolRuntime};
use crate::{JobContext, Tool, ToolOutput};

// Generate component model bindings from the WIT file.
wasmtime::component::bindgen!({
    path: "wit/tool.wit",
    world: "sandboxed-tool",
    async: false,
    with: {},
});

// Alias the export interface types for convenience.
use exports::near::agent::tool as wit_tool;

struct StoreData {
    limiter: WasmResourceLimiter,
    wasi: WasiCtx,
    table: ResourceTable,
    host_state: HostState,
}

struct HostState {
    _capabilities: Capabilities,
    _logs: Vec<LogEntry>,
}

struct LogEntry {
    _level: String,
    _message: String,
}

impl HostState {
    fn new(capabilities: Capabilities) -> Self {
        Self {
            _capabilities: capabilities,
            _logs: Vec::new(),
        }
    }
}

impl StoreData {
    fn new(memory_limit: u64, capabilities: Capabilities) -> Self {
        let wasi = WasiCtxBuilder::new().build();
        Self {
            limiter: WasmResourceLimiter::new(memory_limit),
            wasi,
            table: ResourceTable::new(),
            host_state: HostState::new(capabilities),
        }
    }
}

impl WasiView for StoreData {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl near::agent::host::Host for StoreData {
    fn log(&mut self, level: near::agent::host::LogLevel, message: String) {
        let level_str = format!("{:?}", level);
        self.host_state._logs.push(LogEntry { _level: level_str, _message: message });
    }

    fn now_millis(&mut self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn workspace_read(&mut self, _path: String) -> Option<String> {
        None // Simplified for now
    }

    fn http_request(
        &mut self,
        _method: String,
        _url: String,
        _headers_json: String,
        _body: Option<Vec<u8>>,
        _timeout_ms: Option<u32>,
    ) -> Result<near::agent::host::HttpResponse, String> {
        Err("HTTP not implemented in security bridge yet".to_string())
    }

    fn tool_invoke(&mut self, _alias: String, _params_json: String) -> Result<String, String> {
        Err("Tool invoke not implemented".to_string())
    }

    fn secret_exists(&mut self, _name: String) -> bool {
        false
    }
}

pub struct WasmToolWrapper {
    runtime: Arc<WasmToolRuntime>,
    prepared: Arc<PreparedModule>,
    capabilities: Capabilities,
}

impl WasmToolWrapper {
    pub fn new(
        runtime: Arc<WasmToolRuntime>,
        prepared: Arc<PreparedModule>,
        capabilities: Capabilities,
    ) -> Self {
        Self {
            runtime,
            prepared,
            capabilities,
        }
    }

    fn add_host_functions(linker: &mut Linker<StoreData>) -> Result<(), WasmError> {
        wasmtime_wasi::add_to_linker_sync(linker)
            .map_err(|e| WasmError::ConfigError(e.to_string()))?;

        near::agent::host::add_to_linker(linker, |state| state)
            .map_err(|e| WasmError::ConfigError(e.to_string()))?;

        Ok(())
    }

    fn execute_sync(
        &self,
        params: serde_json::Value,
        context_json: Option<String>,
    ) -> Result<String, WasmError> {
        let engine = self.runtime.engine();
        let limits = &self.prepared.limits;

        let store_data = StoreData::new(limits.memory_bytes, self.capabilities.clone());
        let mut store = Store::new(engine, store_data);

        if self.runtime.config().fuel_config.enabled {
            store.set_fuel(limits.fuel)
                .map_err(|e| WasmError::ConfigError(e.to_string()))?;
        }

        store.epoch_deadline_trap();
        let ticks = (limits.timeout.as_millis() / EPOCH_TICK_INTERVAL.as_millis()).max(1) as u64;
        store.set_epoch_deadline(ticks);
        store.limiter(|data| &mut data.limiter);

        let component = self.prepared.component().clone();
        let mut linker = Linker::new(engine);
        Self::add_host_functions(&mut linker)?;

        let instance = SandboxedTool::instantiate(&mut store, &component, &linker)
            .map_err(|e| WasmError::InstantiationFailed(e.to_string()))?;

        let tool_iface = instance.near_agent_tool();
        let params_json = serde_json::to_string(&params)
            .map_err(|e| WasmError::InvalidResponseJson(e.to_string()))?;

        let request = wit_tool::Request {
            params: params_json,
            context: context_json,
        };

        let response = tool_iface.call_execute(&mut store, &request).map_err(|e| {
            WasmError::Trapped(e.to_string())
        })?;

        if let Some(err) = response.error {
            return Err(WasmError::ToolReturnedError { message: err, hint: String::new() });
        }

        Ok(response.output.unwrap_or_default())
    }
}

pub(super) fn extract_wasm_metadata(
    engine: &wasmtime::Engine,
    component: &wasmtime::component::Component,
    limits: &ResourceLimits,
) -> Result<(String, serde_json::Value), WasmError> {
    let store_data = StoreData::new(limits.memory_bytes, Capabilities::default());
    let mut store = Store::new(engine, store_data);

    store.epoch_deadline_trap();
    store.set_epoch_deadline(10);
    store.limiter(|data| &mut data.limiter);

    let mut linker = Linker::new(engine);
    WasmToolWrapper::add_host_functions(&mut linker)?;
    
    let instance = SandboxedTool::instantiate(&mut store, component, &linker)
        .map_err(|e| WasmError::InstantiationFailed(e.to_string()))?;
    
    let tool_iface = instance.near_agent_tool();

    let description = tool_iface.call_description(&mut store)
        .unwrap_or_else(|_| "WASM tool".to_string());

    let schema = tool_iface.call_schema(&mut store)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .unwrap_or_else(|| serde_json::json!({"type": "object"}));

    Ok((description, schema))
}

#[async_trait::async_trait]
impl Tool for WasmToolWrapper {
    fn name(&self) -> &str {
        &self.prepared.name
    }

    fn description(&self) -> &str {
        &self.prepared.description
    }

    fn parameters_schema(&self) -> serde_json::Value {
        self.prepared.schema.clone()
    }

    async fn execute(&self, params: serde_json::Value, ctx: &JobContext) -> Result<ToolOutput, ToolError> {
        let start = Instant::now();
        let timeout = self.prepared.limits.timeout;

        let context_json = serde_json::to_string(ctx).ok();
        let wrapper = Arc::new(Self::new(Arc::clone(&self.runtime), Arc::clone(&self.prepared), self.capabilities.clone()));

        let result = tokio::time::timeout(timeout, async move {
            tokio::task::spawn_blocking(move || {
                wrapper.execute_sync(params, context_json)
            })
            .await
            .map_err(|e| WasmError::ExecutionPanicked(e.to_string()))?
        })
        .await;

        let duration = start.elapsed();

        match result {
            Ok(Ok(output_json)) => {
                let result: serde_json::Value = serde_json::from_str(&output_json)
                    .unwrap_or(serde_json::Value::String(output_json));
                Ok(ToolOutput::success(result, duration))
            }
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err(WasmError::Timeout(timeout).into()),
        }
    }
}
