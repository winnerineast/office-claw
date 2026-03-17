//! Simplified WASM tool loader for OfficeClaw.

use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use crate::error::WasmError;
use crate::runtime::WasmToolRuntime;
use crate::wrapper::WasmToolWrapper;
use crate::capabilities::Capabilities;

#[derive(Debug, thiserror::Error)]
pub enum WasmLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("WASM compilation error: {0}")]
    Compilation(#[from] WasmError),
}

pub struct WasmToolLoader {
    runtime: Arc<WasmToolRuntime>,
}

impl WasmToolLoader {
    pub fn new(runtime: Arc<WasmToolRuntime>) -> Self {
        Self { runtime }
    }

    pub async fn load_tool(&self, name: &str, wasm_path: &Path) -> Result<WasmToolWrapper, WasmLoadError> {
        let wasm_bytes = fs::read(wasm_path).await?;
        let prepared = self.runtime.prepare(name, &wasm_bytes, None).await?;
        
        // In a real scenario, we'd load capabilities from a .json file
        let capabilities = Capabilities::default();
        
        Ok(WasmToolWrapper::new(Arc::clone(&self.runtime), prepared, capabilities))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::WasmRuntimeConfig;

    #[tokio::test]
    async fn test_loader_init() {
        let config = WasmRuntimeConfig::for_testing();
        let runtime = Arc::new(WasmToolRuntime::new(config).unwrap());
        let _loader = WasmToolLoader::new(runtime);
    }
}
