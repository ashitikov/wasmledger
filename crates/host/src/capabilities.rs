use wasmtime::component::Linker;
use crate::engine::GlobalComponentState;

pub(crate) mod sql;

/// Add capabilities to the linker
pub fn add_to_linker(linker: &mut Linker<GlobalComponentState>) -> anyhow::Result<()> {
    // Add SQL capability
    wasmledger_sql::CoreHost::add_to_linker::<
        GlobalComponentState,
        wasmledger_sql::CoreComponentState,
    >(linker, |s| &mut s.sql)?;

    // Future capabilities will be added here

    Ok(())
}