use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;

use crate::plugin::registry::PluginEntry;

/// Configuration for function execution (limits and timeouts)
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ExecutionConfig {
    /// Fuel limit (computational budget)
    /// Default: 1,000,000,000
    pub fuel_limit: Option<u64>,

    /// Memory limit in bytes
    /// Default: 100 MB
    pub memory_limit_bytes: Option<usize>,

    /// Execution timeout in seconds
    /// Default: 30 seconds
    pub timeout_seconds: u64,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            fuel_limit: Some(1_000_000_000),
            memory_limit_bytes: Some(100 * 1024 * 1024),
            timeout_seconds: 30,
        }
    }
}

impl ExecutionConfig {
    /// Get timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }
}

/// Main host configuration
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HostConfig {
    /// Plugins to load at startup
    pub plugins: Vec<PluginEntry>,

    /// Execution configuration for functions
    pub execution: ExecutionConfig,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            plugins: Vec::new(),
            execution: ExecutionConfig::default(),
        }
    }
}

impl HostConfig {
    /// Load configuration from YAML file
    ///
    /// Reads from `config.yaml` in the current directory by default,
    /// or from the path specified in the `CONFIG_PATH` environment variable.
    pub fn load() -> anyhow::Result<Self> {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());

        // Check if config file exists
        let path = PathBuf::from(&config_path);
        if !path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }

        // Read and parse YAML
        let config_content = std::fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file {}: {}", config_path, e))?;

        let config: Self = serde_yaml::from_str(&config_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse YAML config: {}", e))?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_full_config() {
        let yaml = r#"
plugins:
  - id: money
    path: ./target/wasm32-wasip2/wasmledger_money.wasm
  - id: core
    path: ./target/wasm32-wasip2/wasmledger_core.wasm

execution:
  fuel_limit: 2000000000
  memory_limit_bytes: 209715200
  timeout_seconds: 60
"#;

        let config: HostConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.plugins.len(), 2);
        assert_eq!(config.plugins[0].id, "money");
        assert_eq!(config.execution.fuel_limit, Some(2_000_000_000));
        assert_eq!(config.execution.memory_limit_bytes, Some(200 * 1024 * 1024));
        assert_eq!(config.execution.timeout_seconds, 60);
    }

    #[test]
    fn test_default_execution_limits() {
        let yaml = r#"
plugins:
  - id: money
    path: ./money.wasm
"#;
        let config: HostConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.execution.fuel_limit, Some(1_000_000_000));
        assert_eq!(config.execution.timeout_seconds, 30);
    }

    #[test]
    fn test_empty_config() {
        let yaml = r#"{}"#;
        let config: HostConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.plugins.len(), 0);
        assert!(config.execution.fuel_limit.is_some());
    }
}
