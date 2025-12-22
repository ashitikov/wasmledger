wit_bindgen::generate!({
    path: ["../wit/sql", "../wit/sql/postgres"],
    world: "wasmledger:sql-postgres/postgres",
    with: {
      "wasmledger:sql/query-types": crate::core::bindings::exports::wasmledger::sql::query_types,
      "wasmledger:sql/util-types": crate::core::bindings::wasmledger::sql::util_types,
      "wasmledger:sql/codecs": generate,
    },
});

pub(crate) struct BindingsImpl;

export!(BindingsImpl);

mod codecs;
mod codecs_ext;
mod utils;
