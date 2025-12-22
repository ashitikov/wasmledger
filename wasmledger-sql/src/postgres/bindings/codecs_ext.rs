use crate::{
    core::bindings::{
        exports::wasmledger::sql::{query::QueryResults, query_types::SqlArguments},
        wasmledger::sql::util_types::Error,
    },
    postgres::bindings::{
        BindingsImpl,
        exports::wasmledger::sql_postgres::postgres_codecs_ext::Hstore,
        utils::CodecsUtils,
        wasmledger::sql::codecs::{PushResult, ValuePosition},
    },
};
use sqlx::postgres::types::PgHstore;

impl crate::postgres::bindings::exports::wasmledger::sql_postgres::postgres_codecs_ext::Guest
    for BindingsImpl
{
    fn push_hstore(value: Option<Hstore>, to: &SqlArguments) -> PushResult {
        let value = value.map(|v| PgHstore(v.into_iter().collect()));

        CodecsUtils::encode(value, to)
    }

    fn get_hstore(result: &QueryResults, position: ValuePosition) -> Result<Option<Hstore>, Error> {
        let value: Option<PgHstore> = CodecsUtils::decode(result, position)?;

        Ok(value.map(|v| v.into_iter().collect()))
    }
}
