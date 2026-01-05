use std::sync::Arc;
use wasmtime::component::Component;

pub mod registry;
pub mod migrations;

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
}

impl LoadedPlugin {
    /// Create a new LoadedPlugin
    pub fn new(id: String, component: Component) -> Self {
        Self {
            id,
            component: Arc::new(component),
        }
    }
}

impl std::fmt::Debug for LoadedPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedPlugin")
            .field("id", &self.id)
            .field("component", &"<Component>")
            .finish()
    }
}
