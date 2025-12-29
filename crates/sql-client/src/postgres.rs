pub mod bindings {
    wit_bindgen::generate!({
        path: ["../../wit/sql", "../../wit/sql/postgres", "./wit/postgres"],
        world: "wasmledger:sql-client/client-postgres",
        with: {
          "wasmledger:sql/query-types": crate::base::bindings::wasmledger::sql::query_types,
          "wasmledger:sql/util-types": crate::base::bindings::wasmledger::sql::util_types,
          "wasmledger:sql/codecs": crate::base::bindings::wasmledger::sql::codecs,
          "wasmledger:sql-postgres/postgres-codecs": generate,
          "wasmledger:sql-postgres/postgres-codecs-ext": generate
        }
    });
}