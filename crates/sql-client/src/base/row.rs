use crate::base::{
    bindings::wasmledger::sql::query_types::QueryResults,
    column::{RowColumnPointer, ToColumnIndex},
};

pub struct RowPointer<'a> {
    pub(crate) query_results: &'a QueryResults,
    pub(crate) row_index: u64,
}

impl<'a> RowPointer<'a> {
    pub fn column<T: ToColumnIndex>(&'a self, index: T) -> RowColumnPointer<'a> {
        RowColumnPointer {
            row_pointer: &self,
            column: index.to_column_index(),
        }
    }
}
