use std::sync::Arc;
use wasmtime::{Store, component::{Component, Linker}};
use serde_json::Value;

use crate::{
    app_state::AppState,
    engine::create_core_state,
    capabilities,
    config::ExecutionConfig,
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
        let store_state = create_core_state()
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

        // 4. Clone plugin linker and add capabilities
        // let mut linker = self.app_state.plugin_registry.get_linker().clone();
        let mut linker = Linker::new(&self.app_state.engine);
        capabilities::add_to_linker(&mut linker)
            .map_err(|e| ExecutionError::Internal(e))?;

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
