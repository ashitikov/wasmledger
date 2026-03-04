pub mod bindings {
    wit_bindgen::generate!({
        path: "./wit",
        world: "wasmledger:sql-client/client-base",
        generate_unused_types: true,
        generate_all,
    });
}

impl From<bindings::wasm_sql::core::util_types::Error> for String {
    fn from(value: bindings::wasm_sql::core::util_types::Error) -> Self {
        format!("{value:?}")
    }
}

pub mod column;
pub mod decode;
pub mod iter;
pub mod row;
pub mod derive;
