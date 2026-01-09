use crate::base::{
    bindings::wasmledger::sql::{
        codecs::ValuePosition, query_types::QueryResults, util_types::Error,
    },
    column::RowColumnPointer,
};

pub trait DecodeRequired<T> {
    fn required(self) -> Result<T, Error>;
}

impl<T> DecodeRequired<T> for Result<Option<T>, Error> {
    fn required(self) -> Result<T, Error> {
        self?.ok_or_else(|| Error::Decode("found null".to_string()))
    }
}

impl<'a> RowColumnPointer<'a> {
    pub fn decode<T, F>(&self, f: F) -> Result<Option<T>, Error>
    where
        F: FnOnce(&QueryResults, &ValuePosition) -> Result<Option<T>, Error>,
    {
        let position = (self.row_pointer.row_index, self.column.clone());
        f(&self.row_pointer.query_results, &position)
    }
}
