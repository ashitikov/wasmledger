use std::sync::Arc;
use wasmtime::{Store, component::{Component, Linker}};
use serde_json::Value;

use crate::{
    app_state::AppState,
    engine::create_store_state,
    capabilities,
    plugin_registry::ExecutionConfig,
};

use super::error::ExecutionError;

/// Result of function execution
#[derive(Debug, serde::Serialize)]
pub struct ExecutionResult {
    /// Output from the function (JSON serialized)
    pub output: Value,
    /// Fuel consumed during execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_consumed: Option<u64>,
}

/// Function executor for running ledger functions
pub struct FunctionExecutor {
    app_state: Arc<AppState>,
}

impl FunctionExecutor {
    /// Create a new function executor
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    /// Add plugin exports to linker by instantiating plugins in the store
    ///
    /// TODO (Phase 3): Implement plugin export linking
    ///
    /// The challenge with wasmtime's Component Model:
    /// 1. `LinkerInstance` doesn't have a method to add an already-instantiated component
    /// 2. We need to manually add each exported function using `func_wrap()`
    /// 3. This requires knowing the plugin's WIT interface at compile-time
    ///
    /// Possible solutions:
    /// A. Use `bindgen!` macro for each plugin (requires WIT files at compile-time)
    /// B. Manually introspect Instance exports and add them to LinkerInstance
    /// C. Use runtime composition with custom host functions that delegate to plugins
    ///
    /// For now, plugins are loaded but not linked. Functions can't import plugin interfaces yet.
    /// This will be implemented in Phase 3 once we have concrete WIT interfaces for plugins.
    async fn add_plugin_exports_to_linker(
        &self,
        _linker: &mut Linker<crate::engine::GlobalComponentState>,
        _store: &mut Store<crate::engine::GlobalComponentState>,
    ) -> anyhow::Result<()> {
        if self.app_state.plugin_registry.count() > 0 {
            tracing::debug!(
                "Plugin export linking not yet implemented - {} loaded plugin(s) available but not linked",
                self.app_state.plugin_registry.count()
            );
        }
        Ok(())
    }

    /// Execute a ledger function
    pub async fn execute_function(
        &self,
        wasm_bytes: Vec<u8>,
        config: &ExecutionConfig,
    ) -> Result<ExecutionResult, ExecutionError> {
        // 1. Compile function
        let component = Component::from_binary(&self.app_state.engine, &wasm_bytes)
            .map_err(|e| ExecutionError::CompilationError(e.to_string()))?;

        // 2. Create isolated Store with fresh state
        let store_state = create_store_state()
            .await
            .map_err(|e| ExecutionError::Internal(e))?;
        let mut store = Store::new(&self.app_state.engine, store_state);

        // 3. Set fuel limit if configured
        let initial_fuel = if let Some(fuel) = config.fuel_limit {
            store
                .set_fuel(fuel)
                .map_err(ExecutionError::Internal)?;
            Some(fuel)
        } else {
            None
        };

        // 4. Create linker and add all capabilities + plugin exports
        let mut linker = Linker::new(&self.app_state.engine);
        capabilities::add_to_linker(&mut linker)
            .map_err(|e| ExecutionError::Internal(e))?;

        // 4b. Add plugin exports to linker
        self.add_plugin_exports_to_linker(&mut linker, &mut store)
            .await
            .map_err(ExecutionError::Internal)?;

        // 5. Instantiate component
        let _instance = linker
            .instantiate_async(&mut store, &component)
            .await
            .map_err(|e| ExecutionError::InstantiationError(e.to_string()))?;

        // 6. Calculate fuel consumed
        let fuel_consumed = if let Some(initial) = initial_fuel {
            let remaining = store
                .get_fuel()
                .map_err(ExecutionError::Internal)?;
            Some(initial - remaining)
        } else {
            None
        };

        // 7. For now, return a placeholder result
        // In Phase 3, we'll actually invoke the function's exported methods
        Ok(ExecutionResult {
            output: serde_json::json!({
                "message": "Function instantiated successfully",
                "note": "Actual invocation will be implemented in Phase 3"
            }),
            fuel_consumed,
        })
    }
}
