mod bindings {
    wit_bindgen::generate!({
        path: ["../../../wit/sql", "../../../wit/sql/postgres", "../../../wit/actor", "./wit"],
        world: "wasmledger:core/core",
        with: {
            "wasmledger:sql/query": wasmledger_sql_client::base::bindings::wasmledger::sql::query,
            "wasmledger:sql/query-types": wasmledger_sql_client::base::bindings::wasmledger::sql::query_types,
            "wasmledger:sql/util-types": wasmledger_sql_client::base::bindings::wasmledger::sql::util_types,
            "wasmledger:sql/transaction": wasmledger_sql_client::base::bindings::wasmledger::sql::transaction,
            "wasmledger:sql/connection": wasmledger_sql_client::base::bindings::wasmledger::sql::connection,
            "wasmledger:sql/codecs": wasmledger_sql_client::base::bindings::wasmledger::sql::codecs,
            "wasmledger:sql-postgres/postgres-codecs": wasmledger_sql_client::postgres::bindings::wasmledger::sql_postgres::postgres_codecs,
        },
    });

    use super::BindingsImpl;
    export!(BindingsImpl);
}

mod migrations;

pub struct BindingsImpl;
