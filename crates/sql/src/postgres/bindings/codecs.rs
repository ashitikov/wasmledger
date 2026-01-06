use crate::{
    core::bindings::{
        BindingsImplState,
        wasmledger::sql::{query::QueryResults, query_types::SqlArguments, util_types::Error},
    },
    postgres::bindings::{
        utils::{self},
        wasmledger::{
            sql::codecs::{PushResult, ValuePosition},
            sql_postgres::postgres_codecs::{Hstore, Uuid},
        },
    },
};
use sqlx::types::JsonRawValue;
use sqlx::{postgres::types::PgHstore, types::Json};

impl crate::postgres::bindings::wasmledger::sql_postgres::postgres_codecs::Host
    for BindingsImplState
{
    fn push_int16(
        &mut self,
        value: Option<i16>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_int16(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<i16>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_int32(
        &mut self,
        value: Option<i32>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_int32(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<i32>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_int64(
        &mut self,
        value: Option<i64>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_int64(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<i64>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_float32(
        &mut self,
        value: Option<f32>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_float32(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<f32>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_float64(
        &mut self,
        value: Option<f64>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_float64(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<f64>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_string(
        &mut self,
        value: Option<wasmtime::component::__internal::String>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_string(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<wasmtime::component::__internal::String>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_bool(
        &mut self,
        value: Option<bool>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        utils::encode(value, self.table.get(&to)?)
    }

    fn get_bool(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<bool>, Error> {
        utils::decode(self.table.get(&result)?, position)
    }

    fn push_json(
        &mut self,
        value: Option<wasmtime::component::__internal::String>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        let to = self.table.get(&to)?;
        match value {
            Some(value) => {
                let raw_value =
                    JsonRawValue::from_string(value).map_err(|e| Error::Encode(e.to_string()))?;

                utils::encode(Json(raw_value), to)
            }
            None => utils::encode(value, to),
        }
    }

    fn get_json(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<wasmtime::component::__internal::String>, Error> {
        let a = utils::decode::<Option<&JsonRawValue>>(self.table.get(&result)?, position)?;

        Ok(a.map(|x| x.get().to_string()))
    }

    fn push_uuid(
        &mut self,
        value: Option<Uuid>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        let value = value
            .map(|v| sqlx::types::Uuid::try_parse(v.as_str()))
            .transpose()
            .map_err(|e| Error::Encode(e.to_string()))?;

        utils::encode(value, self.table.get(&to)?)
    }

    fn get_uuid(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<Uuid>, Error> {
        let value: Option<Uuid> = utils::decode(self.table.get(&result)?, position)?;

        Ok(value.map(|v| v.to_string()))
    }

    fn push_hstore(
        &mut self,
        value: Option<Hstore>,
        to: wasmtime::component::Resource<SqlArguments>,
    ) -> PushResult {
        let value = value.map(|v: Vec<(String, Option<String>)>| PgHstore(v.into_iter().collect()));

        utils::encode(value, self.table.get(&to)?)
    }

    fn get_hstore(
        &mut self,
        result: wasmtime::component::Resource<QueryResults>,
        position: ValuePosition,
    ) -> Result<Option<Hstore>, Error> {
        let value: Option<PgHstore> = utils::decode(self.table.get(&result)?, position)?;

        Ok(value.map(|v| v.into_iter().collect()))
    }
}
