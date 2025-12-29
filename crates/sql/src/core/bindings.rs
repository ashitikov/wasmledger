wit_bindgen::generate!({
    path: ["../../wit/sql", "./wit"],
    world: "wasmledger:sql-component/component",
    generate_all
});

pub(crate) struct BindingsImpl;

export!(BindingsImpl);

pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod pool;
pub(crate) mod query;
pub(crate) mod query_results;
pub(crate) mod sql_arguments;
pub(crate) mod transaction;
