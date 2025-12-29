use sqlx::Executor;
use sqlx::any::AnyQueryResult;

use crate::core::bindings::BindingsImpl;
use crate::core::bindings::context::BindingsContext;
use crate::core::bindings::exports::wasmledger::sql::query::QueryExecutor;
use crate::core::bindings::exports::wasmledger::sql::query_types::{
    QueryResults, SqlQuery, SqlString,
};
use crate::core::bindings::query_results::QueryResultsImpl;
use crate::core::bindings::sql_arguments::SqlArgumentsImpl;
use crate::core::bindings::transaction::TransactionImpl;
use crate::core::bindings::wasmledger::sql::util_types::Error;

macro_rules! execute_with {
    ($executor:expr, $query:expr, $op:ident) => {{
        match $executor {
            QueryExecutor::Pool => {
                let pool = BindingsContext::get_pool();
                pool.$op($query).await
            }
            QueryExecutor::Transaction(transaction) => {
                let tx_impl: &TransactionImpl = transaction.get();
                let mut tx = tx_impl.tx.write().await;
                tx.$op($query).await
            }
        }
    }};
}

impl crate::core::bindings::exports::wasmledger::sql::query_types::Guest for BindingsImpl {
    type SqlArguments = SqlArgumentsImpl<'static>;
    type QueryResults = QueryResultsImpl;
}

impl crate::core::bindings::exports::wasmledger::sql::query::Guest for BindingsImpl {
    async fn fetch_all(
        query: SqlQuery,
        executor: QueryExecutor<'_>,
    ) -> Result<QueryResults, Error> {
        let args_impl: SqlArgumentsImpl = query.args.into_inner();
        let args = args_impl.args.into_inner();
        let q = sqlx::query_with(&query.sql, args);

        let results = execute_with!(executor, q, fetch_all)?;

        Ok(QueryResults::new(QueryResultsImpl { results }))
    }

    async fn execute(query: SqlQuery, executor: QueryExecutor<'_>) -> Result<u64, Error> {
        let args_impl: SqlArgumentsImpl = query.args.into_inner();
        let args = args_impl.args.into_inner();
        let q = sqlx::query_with(&query.sql, args);

        let results: AnyQueryResult = execute_with!(executor, q, execute)?.into();

        Ok(results.rows_affected())
    }

    async fn execute_raw(query: SqlString, executor: QueryExecutor<'_>) -> Result<(), Error> {
        let q = sqlx::raw_sql(&query);
        execute_with!(executor, q, execute)?;

        Ok(())
    }
}
