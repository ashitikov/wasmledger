use std::sync::Arc;
use axum::{
    extract::State,
    Json,
    extract::Multipart,
};

use crate::{
    app_state::AppState,
    execution::{FunctionExecutor, ExecutionError},
};

/// Handler for POST /execute endpoint
///
/// Accepts multipart/form-data with the following fields:
/// - `wasm` (required): WASM function bytes
/// - `input` (optional): JSON input parameters for the function (future use)
///
/// Returns JSON response with function output and execution metadata.
pub async fn execute_function_handler(
    State(app_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<crate::execution::ExecutionResult>, ExecutionError> {
    let mut wasm_bytes: Option<Vec<u8>> = None;
    let mut _input: Option<String> = None;

    // Parse multipart fields
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ExecutionError::ValidationError(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "wasm" => {
                let data = field.bytes().await.map_err(|e| {
                    ExecutionError::ValidationError(format!("Failed to read WASM bytes: {}", e))
                })?;
                wasm_bytes = Some(data.to_vec());
            }
            "input" => {
                let data = field.text().await.map_err(|e| {
                    ExecutionError::ValidationError(format!("Failed to read input: {}", e))
                })?;
                _input = Some(data);
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    // Validate required field
    let wasm_bytes = wasm_bytes.ok_or_else(|| {
        ExecutionError::ValidationError("Missing required field 'wasm'".to_string())
    })?;

    // TODO: In the future, parse and use input for function invocation

    // Execute function with config from app state
    let executor = FunctionExecutor::new(app_state.clone());
    let result = executor.execute_function(wasm_bytes, &app_state.config.execution).await?;

    // Return JSON response with output and fuel_consumed in body
    Ok(Json(result))
}
