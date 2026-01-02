use std::num::TryFromIntError;

use crate::{
    core::bindings::{
        wasmledger::sql::util_types::Error,
        wasmledger::sql::{query::QueryResults, query_types::SqlArguments},
    },
    postgres::bindings::wasmledger::sql::codecs::{ColumnIndex, PushResult, ValuePosition},
};
use sqlx::{Arguments, Encode, Postgres, Type};
use sqlx::{Decode, Row};

#[allow(dead_code)]
pub(crate) fn encode<T>(value: T, to: &SqlArguments) -> PushResult
where
    T: 'static + Encode<'static, Postgres> + Type<Postgres>,
{
    let mut args = to.args.blocking_write();
    args.add(value).map_err(|e| Error::Encode(e.to_string()))
}

#[allow(dead_code)]
pub(crate) fn decode<'a, R>(result: &'a QueryResults, position: ValuePosition) -> Result<R, Error>
where
    R: Decode<'a, Postgres> + Type<Postgres>,
{
    let row_index = _index_to_usize(position.0)?;
    let row = result.results.get(row_index);

    match row {
        None => Err(Error::Decode(format!(
            "row index {} is out of bounds ",
            row_index
        ))),
        Some(row) => match position.1 {
            ColumnIndex::Index(i) => {
                let i = _index_to_usize(i)?;
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
