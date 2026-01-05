use std::sync::Arc;
use axum::{
    Router,
    routing::post,
};

mod api;
mod config;
mod app_state;
mod capabilities;
mod engine;
mod execution;
mod plugin;

use crate::{
    app_state::AppState,
    api::execute_function_handler,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize application state (now async)
    let app_state = Arc::new(AppState::new().await?);

    // Log loaded plugins
    let plugin_count = app_state.plugin_registry.count();
    if plugin_count > 0 {
        println!("Loaded {} plugin(s):", plugin_count);
        for plugin in app_state.plugin_registry.all() {
            println!("  - {}", plugin.id);
        }
    } else {
        println!("No plugins loaded (create config.yaml to load plugins)");
    }

    // Build router with routes
    let app = Router::new()
        .route("/execute", post(execute_function_handler))
        .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server listening on http://127.0.0.1:3000");
    println!("POST /execute - Execute a ledger function (multipart/form-data)");
    println!("  - field 'wasm': WASM function bytes (required)");
    println!("  - field 'input': JSON parameters (optional, future use)");

    axum::serve(listener, app).await?;

    Ok(())
}
