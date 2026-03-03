use wasm_runtime_composer::ResourceProxyView;
use wasm_sql::SqlHostStateView;
use wasmtime::{Config, Engine};
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtxView, WasiView};

use crate::capabilities::{
    postgres::{PostgresState, create_postgres_state},
    wasi::{WasiState, create_wasi_state},
};

pub struct CoreState {
    pub postgres: PostgresState,
    pub wasi: WasiState,
    pub proxy_table: ResourceTable,
}

pub fn create_engine() -> anyhow::Result<Engine> {
    let mut config = Config::new();

    config.wasm_component_model(true);
    config.wasm_component_model_async(true);

    Ok(Engine::new(&config)?)
}

/// Create core state (sync, requires postgres initialized)
pub fn create_core_state() -> CoreState {
    let postgres = create_postgres_state();
    let wasi = create_wasi_state();

    CoreState {
        postgres,
        wasi,
        proxy_table: ResourceTable::new(),
    }
}

impl WasiView for CoreState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        self.wasi.to_view()
    }
}

impl SqlHostStateView for CoreState {
    fn sql_host_state(&mut self) -> &mut wasm_sql::SqlHostState {
        &mut self.postgres
    }
}

impl ResourceProxyView for CoreState {
    fn proxy_table(&mut self) -> &mut ResourceTable {
        &mut self.proxy_table
    }
}
