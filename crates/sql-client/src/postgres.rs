pub mod bindings {
    wit_bindgen::generate!({
        path: "./wit",
        world: "wasmledger:sql-client/client-postgres",
        with: {
          "wasm-sql:core/query-types@0.1.0": crate::base::bindings::wasm_sql::core::query_types,
          "wasm-sql:core/util-types@0.1.0": crate::base::bindings::wasm_sql::core::util_types,
          "wasm-sql:core/codecs@0.1.0": crate::base::bindings::wasm_sql::core::codecs,
          "wasm-sql:postgres/codecs@0.1.0": generate,
        }
    });
}