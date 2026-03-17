//! Extended capabilities for WASM sandbox in OfficeClaw.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Rate limiting configuration for WASM tool calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_requests_per_minute: u32,
    pub max_bytes_per_minute: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60,
            max_bytes_per_minute: 10 * 1024 * 1024,
        }
    }
}

/// All capabilities that can be granted to a WASM tool.
#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    pub workspace_read: Option<WorkspaceCapability>,
    pub http: Option<HttpCapability>,
    pub tool_invoke: Option<ToolInvokeCapability>,
    pub secrets: Option<SecretsCapability>,
}

impl Capabilities {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn with_workspace_read(mut self, prefixes: Vec<String>) -> Self {
        self.workspace_read = Some(WorkspaceCapability {
            allowed_prefixes: prefixes,
            reader: None,
        });
        self
    }

    pub fn with_http(mut self, http: HttpCapability) -> Self {
        self.http = Some(http);
        self
    }

    pub fn with_secrets(mut self, allowed: Vec<String>) -> Self {
        self.secrets = Some(SecretsCapability {
            allowed_names: allowed,
        });
        self
    }
}

#[derive(Clone, Default)]
pub struct WorkspaceCapability {
    pub allowed_prefixes: Vec<String>,
    pub reader: Option<Arc<dyn WorkspaceReader>>,
}

impl std::fmt::Debug for WorkspaceCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceCapability")
            .field("allowed_prefixes", &self.allowed_prefixes)
            .field("reader", &self.reader.is_some())
            .finish()
    }
}

pub trait WorkspaceReader: Send + Sync {
    fn read(&self, path: &str) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct HttpCapability {
    pub allowlist: Vec<EndpointPattern>,
    pub rate_limit: RateLimitConfig,
    pub max_request_bytes: usize,
    pub max_response_bytes: usize,
    pub timeout: Duration,
}

impl Default for HttpCapability {
    fn default() -> Self {
        Self {
            allowlist: Vec::new(),
            rate_limit: RateLimitConfig::default(),
            max_request_bytes: 1024 * 1024,
            max_response_bytes: 10 * 1024 * 1024,
            timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointPattern {
    pub host: String,
    pub path_prefix: Option<String>,
    pub methods: Vec<String>,
}

impl EndpointPattern {
    pub fn host(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            path_prefix: None,
            methods: Vec::new(),
        }
    }

    pub fn matches(&self, url_host: &str, url_path: &str, method: &str) -> bool {
        if !self.host_matches(url_host) {
            return false;
        }

        if let Some(ref prefix) = self.path_prefix {
            if !url_path.starts_with(prefix) {
                return false;
            }
        }

        if !self.methods.is_empty() {
            let method_upper = method.to_uppercase();
            if !self.methods.iter().any(|m| m.to_uppercase() == method_upper) {
                return false;
            }
        }

        true
    }

    pub fn host_matches(&self, url_host: &str) -> bool {
        if self.host == url_host {
            return true;
        }

        if let Some(suffix) = self.host.strip_prefix("*.") {
            if url_host.ends_with(suffix) && url_host.len() > suffix.len() {
                let prefix = &url_host[..url_host.len() - suffix.len()];
                if prefix.ends_with('.') || prefix.is_empty() {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct ToolInvokeCapability {
    pub aliases: HashMap<String, String>,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Default)]
pub struct SecretsCapability {
    pub allowed_names: Vec<String>,
}

impl SecretsCapability {
    pub fn is_allowed(&self, name: &str) -> bool {
        for pattern in &self.allowed_names {
            if pattern == name {
                return true;
            }
            if let Some(prefix) = pattern.strip_suffix('*') {
                if name.starts_with(prefix) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_default_is_none() {
        let caps = Capabilities::default();
        assert!(caps.workspace_read.is_none());
        assert!(caps.http.is_none());
    }

    #[test]
    fn test_endpoint_pattern_exact_host() {
        let pattern = EndpointPattern::host("api.example.com");
        assert!(pattern.matches("api.example.com", "/", "GET"));
        assert!(!pattern.matches("other.example.com", "/", "GET"));
    }

    #[test]
    fn test_endpoint_pattern_wildcard_host() {
        let pattern = EndpointPattern::host("*.example.com");
        assert!(pattern.matches("api.example.com", "/", "GET"));
        assert!(pattern.matches("sub.api.example.com", "/", "GET"));
        assert!(!pattern.matches("example.com", "/", "GET"));
    }

    #[test]
    fn test_secrets_capability_glob() {
        let cap = SecretsCapability {
            allowed_names: vec!["openai_*".to_string()],
        };
        assert!(cap.is_allowed("openai_key"));
        assert!(!cap.is_allowed("anthropic_key"));
    }
}
