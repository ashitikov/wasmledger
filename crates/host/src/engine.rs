use wasmtime::{Config, Engine};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::capabilities::{
    postgres::{PostgresState, create_postgres_state},
    wasi::{WasiState, create_wasi_state},
};

/// State holding all host capabilities available to WASM components
pub struct CoreState {
    pub postgres: PostgresState,
    pub wasi: WasiState,
}

/// Create a configured Wasmtime engine with component model support
pub fn create_engine() -> anyhow::Result<Engine> {
    let mut config = Config::new();

    // Enable component model
    config.wasm_component_model(true);

    // Enable async support
    config.async_support(true);
    config.wasm_component_model(true);
    config.wasm_component_model_async(true);
    // config.wasm_compo
    // config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    // config.wasm_component_model_async_builtins(true);
    // config.wasm_component_model_async_stackful(true);
    // config.wasm_component_model_error_context(true);
    // config.wasm_component_model_threading(true);

    // Enable fuel consumption tracking
    // config.consume_fuel(true);

    // Enable epoch interruption for timeouts
    // config.epoch_interruption(true);

    // Enable copy-on-write memory for efficiency
    // config.memory_init_cow(true);

    Ok(Engine::new(&config)?)
}

pub async fn create_core_state() -> anyhow::Result<CoreState> {
    let postgres = create_postgres_state().await?;
    let wasi = create_wasi_state();

    Ok(CoreState { postgres, wasi })
}

impl WasiView for CoreState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        self.wasi.to_view()
    }
}
