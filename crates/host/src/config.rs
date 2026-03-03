use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;

/// Single extension entry in configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionEntry {
    pub id: String,
    pub path: PathBuf,
}

/// Configuration for function execution (limits and timeouts)
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ExecutionConfig {
    pub fuel_limit: Option<u64>,
    pub memory_limit_bytes: Option<usize>,
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
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }
}

/// Main host configuration
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HostConfig {
    pub extensions: Vec<ExtensionEntry>,
    pub execution: ExecutionConfig,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            execution: ExecutionConfig::default(),
        }
    }
}

impl HostConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());

        let path = PathBuf::from(&config_path);
        if !path.exists() {
            return Ok(Self::default());
        }

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
extensions:
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
        assert_eq!(config.extensions.len(), 2);
        assert_eq!(config.extensions[0].id, "money");
        assert_eq!(config.execution.fuel_limit, Some(2_000_000_000));
        assert_eq!(config.execution.memory_limit_bytes, Some(200 * 1024 * 1024));
        assert_eq!(config.execution.timeout_seconds, 60);
    }

    #[test]
    fn test_default_execution_limits() {
        let yaml = r#"
extensions:
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
        assert_eq!(config.extensions.len(), 0);
        assert!(config.execution.fuel_limit.is_some());
    }
}
