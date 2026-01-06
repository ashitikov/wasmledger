use crate::{
    capabilities::{self},
    engine::CoreState,
};
use wasmtime::component::Linker;

pub(crate) mod postgres;
pub(crate) mod wasi;

/// Add capabilities to the linker
pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
    capabilities::wasi::add_to_linker(linker)?;
    capabilities::postgres::add_to_linker(linker)?;

    Ok(())
}
