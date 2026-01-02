use sqlx::any::AnyQueryResult;

use crate::core::bindings::BindingsImplState;
use crate::core::bindings::executor::{ErasedExecutor, QueryOrRaw};
use crate::core::bindings::query_results::QueryResultsImpl;
use crate::core::bindings::wasmledger::sql::query::QueryExecutor;
use crate::core::bindings::wasmledger::sql::query_types::{QueryResults, SqlQuery, SqlString};
use crate::core::bindings::wasmledger::sql::util_types::Error;

impl crate::core::bindings::wasmledger::sql::query::Host for BindingsImplState {}
impl crate::core::bindings::wasmledger::sql::query_types::Host for BindingsImplState {}

impl crate::core::bindings::wasmledger::sql::query::HostWithStore for BindingsImplState {
    async fn fetch_all<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        query: SqlQuery,
        executor: QueryExecutor,
    ) -> Result<wasmtime::component::Resource<QueryResults>, Error> {
        let results = executor
            .fetch_all(QueryOrRaw::Query(query), accessor)
            .await?;

        let query_results =
            accessor.with(|mut access| access.get().table.push(QueryResultsImpl { results }))?;

        Ok(query_results)
    }

    async fn execute<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        query: SqlQuery,
        executor: QueryExecutor,
    ) -> Result<u64, Error> {
        let results: AnyQueryResult = executor
            .execute(QueryOrRaw::Query(query), accessor)
            .await?
            .into();

        Ok(results.rows_affected())
    }

    async fn execute_raw<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        query: SqlString,
        executor: QueryExecutor,
    ) -> Result<(), Error> {
        executor.execute(QueryOrRaw::Raw(query), accessor).await?;

        Ok(())
    }
}
