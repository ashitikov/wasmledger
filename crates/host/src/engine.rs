use wasmtime::{Config, Engine};

use crate::capabilities::sql::create_sql_state;

/// Global component state shared across WASM component instances
pub struct GlobalComponentState {
    pub sql: wasmledger_sql::CoreComponentState,
}

/// Create a configured Wasmtime engine with component model support
pub fn create_engine() -> anyhow::Result<Engine> {
    let mut config = Config::new();

    // Enable component model
    config.wasm_component_model(true);

    // Enable async support
    config.async_support(true);

    // Enable fuel consumption tracking
    config.consume_fuel(true);

    // Enable epoch interruption for timeouts
    config.epoch_interruption(true);

    // Enable copy-on-write memory for efficiency
    config.memory_init_cow(true);

    Ok(Engine::new(&config)?)
}

/// Create store state with SQL capability
pub async fn create_store_state() -> anyhow::Result<GlobalComponentState> {
    let sql = create_sql_state().await?;
    Ok(GlobalComponentState { sql })
}
