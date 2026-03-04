mod bindings {
    wit_bindgen::generate!({
        path: "./wit",
        world: "wasmledger:ext-idgen-pg/idgen-pg",
        with: {
            "wasm-sql:core/query@0.1.0": wasmledger_sql_client::base::bindings::wasm_sql::core::query,
            "wasm-sql:core/query-types@0.1.0": wasmledger_sql_client::base::bindings::wasm_sql::core::query_types,
            "wasm-sql:core/util-types@0.1.0": wasmledger_sql_client::base::bindings::wasm_sql::core::util_types,
            "wasm-sql:core/transaction@0.1.0": wasmledger_sql_client::base::bindings::wasm_sql::core::transaction,
            "wasm-sql:core/connection@0.1.0": wasmledger_sql_client::base::bindings::wasm_sql::core::connection,
            "wasm-sql:core/codecs@0.1.0": wasmledger_sql_client::base::bindings::wasm_sql::core::codecs,
            "wasm-sql:postgres/codecs@0.1.0": wasmledger_sql_client::postgres::bindings::wasm_sql::postgres::codecs,
        },
    });

    use super::BindingsImpl;
    export!(BindingsImpl);
}

mod idgen;
mod migrations;

pub struct BindingsImpl;
