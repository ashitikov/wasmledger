use crate::{
    core::bindings::{
        exports::wasmledger::sql::{query::QueryResults, query_types::SqlArguments},
        wasmledger::sql::util_types::Error,
    },
    postgres::bindings::{
        BindingsImpl,
        exports::wasmledger::sql_postgres::postgres_codecs::Uuid,
        utils::CodecsUtils,
        wasmledger::sql::codecs::{PushResult, ValuePosition},
    },
};
use sqlx::types::Json;
use sqlx::types::JsonRawValue;

impl crate::postgres::bindings::exports::wasmledger::sql_postgres::postgres_codecs::Guest
    for BindingsImpl
{
    fn push_int16(value: Option<i16>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn push_int32(value: Option<i32>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn push_int64(value: Option<i64>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn push_string(value: Option<String>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn push_json(value: Option<String>, to: &SqlArguments) -> PushResult {
        match value {
            Some(value) => {
                let raw_value =
                    JsonRawValue::from_string(value).map_err(|e| Error::Encode(e.to_string()))?;

                CodecsUtils::encode(Json(raw_value), to)
            }
            None => CodecsUtils::encode(value, to),
        }
    }

    fn get_int16(result: &QueryResults, position: ValuePosition) -> Result<Option<i16>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn get_int32(result: &QueryResults, position: ValuePosition) -> Result<Option<i32>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn get_int64(result: &QueryResults, position: ValuePosition) -> Result<Option<i64>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn get_string(result: &QueryResults, position: ValuePosition) -> Result<Option<String>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn get_json(result: &QueryResults, position: ValuePosition) -> Result<Option<String>, Error> {
        let a = CodecsUtils::decode::<Option<&JsonRawValue>>(result, position)?;

        Ok(a.map(|x| x.get().to_string()))
    }

    fn push_bool(value: Option<bool>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn get_bool(result: &QueryResults, position: ValuePosition) -> Result<Option<bool>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn push_float32(value: Option<f32>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn get_float32(result: &QueryResults, position: ValuePosition) -> Result<Option<f32>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn push_float64(value: Option<f64>, to: &SqlArguments) -> PushResult {
        CodecsUtils::encode(value, to)
    }

    fn get_float64(result: &QueryResults, position: ValuePosition) -> Result<Option<f64>, Error> {
        CodecsUtils::decode(result, position)
    }

    fn push_uuid(value: Option<Uuid>, to: &SqlArguments) -> PushResult {
        let value = value
            .map(|v| sqlx::types::Uuid::try_parse(v.as_str()))
            .transpose()
            .map_err(|e| Error::Encode(e.to_string()))?;

        CodecsUtils::encode(value, to)
    }

    fn get_uuid(result: &QueryResults, position: ValuePosition) -> Result<Option<Uuid>, Error> {
        let value: Option<Uuid> = CodecsUtils::decode(result, position)?;

        Ok(value.map(|v| v.to_string()))
    }
}
