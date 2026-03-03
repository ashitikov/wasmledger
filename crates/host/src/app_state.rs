use std::sync::Arc;
use wasmtime::Engine;

use crate::{
    capabilities::postgres,
    config::HostConfig,
    engine::create_engine,
    extension::manager::ExtensionManager,
};

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    /// Wasmtime engine (shared across all requests)
    pub engine: Engine,
    /// Extension manager (loaded at startup)
    pub extension_manager: Arc<ExtensionManager>,
    /// Host configuration
    pub config: Arc<HostConfig>,
}

impl AppState {
    pub async fn initialize() -> anyhow::Result<Self> {
        let engine = create_engine()?;
        let config = HostConfig::load()?;

        postgres::initialize_database().await?;

        let extension_manager =
            ExtensionManager::load_from_config(&engine, &config).await?;

        Ok(Self {
            engine,
            extension_manager: Arc::new(extension_manager),
            config: Arc::new(config),
        })
    }
}
