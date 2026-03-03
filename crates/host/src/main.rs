use axum::{Router, routing::post};
use std::sync::Arc;

mod api;
mod app_state;
mod capabilities;
mod extension;
mod config;
mod engine;
mod execution;
mod migrations;

use crate::{
    api::execute_function_handler,
    app_state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = Arc::new(AppState::initialize().await?);
    let client = app_state.extension_manager.client();

    let extension_count = app_state.extension_manager.count();
    if extension_count > 0 {
        println!("Loaded {} extension(s):", extension_count);
        for id in app_state.extension_manager.extension_ids() {
            println!("  - {}", id);
        }
    } else {
        println!("No extensions loaded (create config.yaml to load extensions)");
    }

    // Apply SQL migrations from extensions
    migrations::apply_from_extensions(client).await?;

    // Lifecycle: on-init
    println!("Dispatching on-init...");
    client.on_init().await?;
    println!("Extensions initialized successfully");

    let app = Router::new()
        .route("/execute", post(execute_function_handler))
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    // Lifecycle: on-shutdown
    println!("Dispatching on-shutdown...");
    client.on_shutdown().await?;

    Ok(())
}
