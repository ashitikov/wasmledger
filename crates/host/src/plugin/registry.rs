use anyhow::Context;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::PathBuf};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};

use crate::config::HostConfig;
use crate::plugin::{LoadedPlugin, client};
use crate::{
    capabilities,
    engine::{CoreState, create_core_state},
};

/// Single plugin entry in configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PluginEntry {
    /// Plugin identifier (e.g., "money", "core", "my-custom-plugin")
    pub id: String,
    /// Path to the .wasm file
    pub path: PathBuf,
}

/// Registry of all loaded plugins
///
/// Plugins are loaded once at startup, compiled into Components, and instantiated.
/// The base linker contains all plugin interfaces (without host capabilities).
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
    pub async fn load_from_config(engine: &Engine, config: &HostConfig) -> anyhow::Result<Self> {
        let mut registry = Self {
            plugins: HashMap::new(),
            engine: engine.clone(),
        };

        // Phase 1: Load all plugin components
        for entry in config.plugins.iter() {
            registry.instantiate_plugin(entry).await?;
        }

        Ok(registry)
    }

    /// Load a single plugin component from a PluginEntry
    async fn instantiate_plugin(&mut self, entry: &PluginEntry) -> anyhow::Result<()> {
        let component = Component::from_file(&self.engine, &entry.path).map_err(|e| {
            anyhow::anyhow!(
                "Failed to compile plugin '{}' from {:?}: {}",
                entry.id,
                entry.path,
                e
            )
        })?;

        let imported_interfaces: HashSet<String> = component
            .component_type()
            .imports(&self.engine)
            .map(|(name, _)| name.to_string())
            .collect();

        // Create store for this plugin
        let store_state = create_core_state().await?;
        let mut store = Store::new(&self.engine, store_state);

        // Clone base linker and add capabilities
        let linker = create_default_linker(&self.engine)?;

        // Instantiate component
        let instance = linker
            .instantiate_async(&mut store, &component)
            .await
            .context(format!("Failed to instantiate plugin '{}'", &entry.id))?;

        // Create LoadedPlugin and store in registry
        let loaded_plugin = LoadedPlugin {
            id: entry.id.clone(),
            component: Arc::new(component),
            imported_interfaces,
            instance: instance,
            store: Mutex::new(store),
        };

        self.plugins.insert(entry.id.clone(), loaded_plugin);

        tracing::debug!(plugin = %entry.id, "Plugin instantiated");

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

fn create_default_linker(engine: &Engine) -> anyhow::Result<Linker<CoreState>> {
    let mut linker = Linker::new(engine);
    linker.allow_shadowing(true);

    client::add_all_clients_to_linker(&mut linker)?;
    capabilities::add_to_linker(&mut linker)?;

    Ok(linker)
}
