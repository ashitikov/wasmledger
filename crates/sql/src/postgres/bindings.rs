use wasmtime::component::{HasData, ResourceTable};

wasmtime::component::bindgen!({
    path: ["../../wit/sql", "../../wit/sql/postgres", "./wit", "./wit/codecs-postgres"],
    world: "wasmledger:sql-host-postgres-codecs/codecs-postgres",
    with: {
      "wasmledger:sql/query-types": crate::core::bindings::wasmledger::sql::query_types,
      "wasmledger:sql/util-types": crate::core::bindings::wasmledger::sql::util_types,
    }
});

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct BindingsImplState {
    table: ResourceTable,
}

impl HasData for BindingsImplState {
    type Data<'a>
        = &'a mut BindingsImplState
    where
        Self: 'a;
}

mod codecs;
mod utils;
