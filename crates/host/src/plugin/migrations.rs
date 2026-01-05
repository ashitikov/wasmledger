pub(crate) mod bindings {
    wasmtime::component::bindgen!({
        path: ["../../wit/sql", "../../wit/plugin", "./wit"],
        world: "wasmledger:plugin-client/client",
        with: {
            "wasmledger:sql/query-types": wasmledger_sql::core::bindings::wasmledger::sql::query_types,
            "wasmledger:sql/util-types": wasmledger_sql::core::bindings::wasmledger::sql::util_types,
            "wasmledger:sql/connection": wasmledger_sql::core::bindings::wasmledger::sql::connection,
            "wasmledger:sql/query": wasmledger_sql::core::bindings::wasmledger::sql::query,
            "wasmledger:sql/transaction": wasmledger_sql::core::bindings::wasmledger::sql::transaction,
        },
        require_store_data_send: true,
    });
}
