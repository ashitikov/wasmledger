use wasmtime::component::Accessor;

use crate::{
    core::bindings::{
        BindingsImplState,
        wasmledger::sql::{
            query::QueryExecutor,
            query_types::{SqlQuery, SqlString},
            util_types::Error,
        },
    },
    sqldb::SqlDatabase,
};

#[allow(dead_code)]
pub enum QueryOrRaw {
    Query(SqlQuery),
    Raw(SqlString),
}

#[macro_export]
macro_rules! execute_with {
    ($executor:expr, $accessor:expr, $query:expr, $op:ident) => {{
        use crate::core::bindings::wasmledger::sql::util_types::Error;
        use sqlx::Executor;

        match $query {
            QueryOrRaw::Query(query) => {
                let query = {
                    let args_resource = query.args;
                    let args = $accessor.with(|mut access| {
                        let st = access.get();
                        st.table.delete(args_resource).map(|x| x.args.into_inner())
                    })?;
                    sqlx::query_with(&query.sql, args).persistent(query.persistent.unwrap_or(true))
                };

                let result = $executor.$op(query).await?;
                Result::<_, Error>::Ok(result)
            }
            QueryOrRaw::Raw(sql) => {
                let query = sqlx::raw_sql(&sql);
                let result = $executor.$op(query).await?;
                Result::<_, Error>::Ok(result)
            }
        }
    }};
}

#[allow(dead_code)]
pub(crate) trait ErasedExecutor<D: wasmtime::component::HasData> {
    async fn fetch_all<T>(
        &self,
        query: QueryOrRaw,
        accessor: &Accessor<T, D>,
    ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error>;

    async fn execute<T>(
        &self,
        query: QueryOrRaw,
        accessor: &Accessor<T, D>,
    ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error>;
}

impl ErasedExecutor<BindingsImplState> for QueryExecutor {
    async fn fetch_all<T>(
        &self,
        query: QueryOrRaw,
        accessor: &Accessor<T, BindingsImplState>,
    ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error> {
        match self {
            QueryExecutor::Pool => {
                let pool = accessor.with(|mut access| access.get().sql_db.pool.clone());
                let rows = execute_with!(pool, accessor, query, fetch_all)?;

                Ok(rows)
            }
            QueryExecutor::Connection(resource) => {
                let conn_impl = accessor.with(|mut access| {
                    let state = access.get();
                    state.table.get(&resource).map(|conn| conn.clone())
                })?;

                conn_impl.fetch_all(query, accessor).await
            }
            QueryExecutor::Transaction(resource) => {
                let tx_impl = accessor.with(|mut access| {
                    let state = access.get();
                    state.table.get(&resource).map(|tx| tx.clone())
                })?;

                tx_impl.fetch_all(query, accessor).await
            }
        }
    }

    async fn execute<T>(
        &self,
        query: QueryOrRaw,
        accessor: &Accessor<T, BindingsImplState>,
    ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error> {
        match self {
            QueryExecutor::Pool => {
                let pool = accessor.with(|mut access| access.get().sql_db.pool.clone());
                let rows = execute_with!(pool, accessor, query, execute)?;
                Ok(rows)
            }
            QueryExecutor::Connection(resource) => {
                let conn_impl = accessor.with(|mut access| {
                    let state = access.get();
                    state.table.get(&resource).map(|conn| conn.clone())
                })?;

                conn_impl.execute(query, accessor).await
            }
            QueryExecutor::Transaction(resource) => {
                let tx_impl = accessor.with(|mut access| {
                    let state = access.get();
                    state.table.get(&resource).map(|tx| tx.clone())
                })?;

                tx_impl.execute(query, accessor).await
            }
        }
    }
}
