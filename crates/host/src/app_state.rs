use std::sync::Arc;
use wasmtime::Engine;

use crate::{config::HostConfig, engine::create_engine, plugin::registry::PluginRegistry};

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
    /// Create new application state with configured engine and loaded plugins
    ///
    /// This is now async because we need to read plugin .wasm files from disk.
    pub async fn new() -> anyhow::Result<Self> {
        let engine = create_engine()?;

        // Load configuration from YAML
        let config = Arc::new(HostConfig::load()?);

        // Load all plugins from configuration
        let plugin_registry =
            PluginRegistry::load_from_config(engine.clone(), config.clone()).await?;

        Ok(Self {
            engine,
            plugin_registry,
            config,
        })
    }
}
