use crate::base::bindings::wasmledger::sql::{query::QueryExecutor};

impl<'a> Clone for QueryExecutor<'a> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a> Copy for QueryExecutor<'a> {}
