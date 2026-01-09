use wasmtime::component::Linker;

use crate::engine::CoreState;

pub(crate) mod bindings {
    wasmtime::component::bindgen!({
        path: ["../../wit/sql", "../../wit/component", "./wit"],
        world: "wasmledger:component-client/component",
    });
}

pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
    Ok(())
}
