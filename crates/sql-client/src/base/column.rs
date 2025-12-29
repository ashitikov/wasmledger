use crate::base::{bindings::wasmledger::sql::codecs::ColumnIndex, row::RowPointer};

pub struct RowColumnPointer<'a> {
    pub(crate) row_pointer: &'a RowPointer<'a>,
    pub(crate) column: ColumnIndex,
}

pub trait ToColumnIndex {
    fn to_column_index(&self) -> ColumnIndex;
}

impl ToColumnIndex for String {
    fn to_column_index(&self) -> ColumnIndex {
        ColumnIndex::ColumnName(self.clone())
    }
}

impl ToColumnIndex for &str {
    fn to_column_index(&self) -> ColumnIndex {
        ColumnIndex::ColumnName(self.to_string())
    }
}

impl ToColumnIndex for u64 {
    fn to_column_index(&self) -> ColumnIndex {
        ColumnIndex::Index(*self)
    }
}
