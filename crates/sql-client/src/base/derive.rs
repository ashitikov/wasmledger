use crate::base::bindings::wasm_sql::core::{query::QueryExecutor};

impl<'a> Clone for QueryExecutor<'a> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a> Copy for QueryExecutor<'a> {}
