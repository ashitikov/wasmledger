use std::sync::Arc;
use wasmtime::Engine;

use crate::{component::registry::PluginRegistry, config::HostConfig, engine::create_engine};

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    /// Wasmtime engine (shared across all requests)
    pub engine: Engine,
    /// Plugin registry (loaded at startup)
    pub plugin_registry: Arc<PluginRegistry>,
    /// Host configuration (plugins, execution limits, etc.)
    pub config: Arc<HostConfig>,
}

impl AppState {
    pub async fn initialize() -> anyhow::Result<Self> {
        let engine = create_engine()?;

        let config = HostConfig::load()?;
        let plugin_registry = PluginRegistry::load_from_config(&engine, &config).await?;

        Ok(Self {
            engine,
            plugin_registry: Arc::new(plugin_registry),
            config: Arc::new(config),
        })
    }
}
