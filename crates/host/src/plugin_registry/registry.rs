use std::collections::HashMap;
use std::sync::Arc;
use wasmtime::{Engine, component::Component};

use super::{
    config::{HostConfig, PluginEntry},
    plugin::LoadedPlugin,
};

/// Registry of all loaded plugins
///
/// Plugins are loaded once at startup and compiled into Components.
/// These Components are then instantiated per-request for isolation.
#[derive(Debug)]
pub struct PluginRegistry {
    /// Map from plugin ID to loaded plugin
    plugins: HashMap<String, LoadedPlugin>,
    /// Wasmtime engine (stored for potential future use)
    engine: Engine,
}

impl PluginRegistry {
    /// Load all plugins from configuration
    ///
    /// This reads .wasm files from disk, compiles them into Components,
    /// and stores them in the registry for future instantiation.
    pub async fn load_from_config(engine: Engine, config: Arc<HostConfig>) -> anyhow::Result<Arc<Self>> {
        let mut registry = Self {
            plugins: HashMap::new(),
            engine: engine.clone(),
        };

        // Load each plugin entry
        for entry in config.plugins.iter() {
            registry.load_plugin(entry).await?;
        }

        Ok(Arc::new(registry))
    }

    /// Load a single plugin from a PluginEntry
    async fn load_plugin(&mut self, entry: &PluginEntry) -> anyhow::Result<()> {
        let component = Component::from_file(&self.engine, &entry.path).map_err(|e| {
            anyhow::anyhow!(
                "Failed to compile plugin '{}' from {:?}: {}",
                entry.id,
                entry.path,
                e
            )
        })?;

        // 3. Create LoadedPlugin and store in registry
        let loaded_plugin = LoadedPlugin::new(entry.id.clone(), component);
        self.plugins.insert(entry.id.clone(), loaded_plugin);

        Ok(())
    }

    /// Get a plugin by ID
    pub fn get(&self, id: &str) -> Option<&LoadedPlugin> {
        self.plugins.get(id)
    }

    /// Iterate over all loaded plugins
    pub fn all(&self) -> impl Iterator<Item = &LoadedPlugin> {
        self.plugins.values()
    }

    /// Get the number of loaded plugins
    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}
