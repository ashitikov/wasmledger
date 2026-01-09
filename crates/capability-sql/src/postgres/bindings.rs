use crate::core::bindings::BindingsImplState;

wasmtime::component::bindgen!({
    path: ["../../wit/sql", "../../wit/sql/postgres", "./wit", "./wit/codecs-postgres"],
    world: "wasmledger:sql-host-postgres-codecs/codecs-postgres",
    with: {
      "wasmledger:sql/query-types": crate::core::bindings::wasmledger::sql::query_types,
      "wasmledger:sql/util-types": crate::core::bindings::wasmledger::sql::util_types,
    }
});

impl crate::postgres::bindings::wasmledger::sql::codecs::Host for BindingsImplState {}

mod codecs;
mod utils;
