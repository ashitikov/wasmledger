use std::num::TryFromIntError;

use crate::{
    core::bindings::{
        exports::wasmledger::sql::{query::QueryResults, query_types::SqlArguments},
        query_results::QueryResultsImpl,
        sql_arguments::SqlArgumentsImpl,
        wasmledger::sql::util_types::Error,
    },
    postgres::bindings::wasmledger::sql::codecs::{ColumnIndex, PushResult, ValuePosition},
};
use sqlx::{Arguments, Encode, Postgres, Type};
use sqlx::{Decode, Row};

pub(crate) struct CodecsUtils;

impl CodecsUtils {
    pub(crate) fn encode<T>(value: T, to: &SqlArguments) -> PushResult
    where
        T: 'static + Encode<'static, Postgres> + Type<Postgres>,
    {
        let _impl: &SqlArgumentsImpl = to.get();
        let mut args = _impl.args.blocking_write();
        args.add(value).map_err(|e| Error::Encode(e.to_string()))
    }

    pub(crate) fn decode<'a, R>(result: &'a QueryResults, position: ValuePosition) -> Result<R, Error>
    where
        R: Decode<'a, Postgres> + Type<Postgres>,
    {
        let _impl: &QueryResultsImpl = result.get();
        let row_index = Self::_index_to_usize(position.0)?;

        let row = _impl.results.get(row_index);

        match row {
            None => Err(Error::Decode("row not found".to_string())),
            Some(row) => match position.1 {
                ColumnIndex::Index(i) => {
                    let i = Self::_index_to_usize(i)?;
                    let r: R = row.try_get(i)?;

                    Ok(r)
                }
                ColumnIndex::ColumnName(name) => {
                    let r: R = row.try_get(name.as_str())?;

                    Ok(r)
                }
            },
        }
    }

    fn _index_to_usize(index: u64) -> Result<usize, Error> {
        index
            .try_into()
            .map_err(|e: TryFromIntError| Error::Decode(e.to_string()))
    }
}
