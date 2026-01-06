use crate::engine::CoreState;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use wasmtime::{
    Store,
    component::{Component, Instance},
};

pub mod client;
pub mod registry;

/// A plugin that has been loaded and compiled, ready for instantiation
pub struct LoadedPlugin {
    /// Plugin identifier from config (e.g., "money", "core", "my-custom-plugin")
    pub id: String,
    /// Pre-compiled WebAssembly component
    ///
    /// This is wrapped in Arc because it's shared across all instantiations.
    /// Components are cheap to instantiate but expensive to compile, so we
    /// compile once at startup and instantiate per-request for isolation.
    pub component: Arc<Component>,

    pub imported_interfaces: HashSet<String>,

    /// Pre-instantiated plugin instance
    ///
    /// Plugins are instantiated once at startup and reused throughout the application lifetime.
    /// This allows plugin interfaces to be called directly without re-instantiation.
    pub instance: Instance,
    /// Store for the plugin instance
    ///
    /// Each plugin gets its own Store with CapabilitiesState.
    /// Mutex provides thread-safe interior mutability for the Store, which must be mutable when executing WASM.
    pub store: Mutex<Store<CoreState>>,
}

impl std::fmt::Debug for LoadedPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedPlugin")
            .field("id", &self.id)
            .field("component", &"<Component>")
            .finish()
    }
}
