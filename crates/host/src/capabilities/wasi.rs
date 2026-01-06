use wasmtime::component::Linker;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView};

use crate::engine::CoreState;

pub(crate) struct WasiState {
    ctx: WasiCtx,
    table: ResourceTable,
}

pub(crate) fn create_wasi_state() -> WasiState {
    let ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let table = ResourceTable::new();

    WasiState { ctx, table }
}

impl WasiState {
    pub fn to_view(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
    wasmtime_wasi::p2::add_to_linker_async(linker)?;

    Ok(())
}
