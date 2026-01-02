use std::sync::Arc;

use sqlx::pool::PoolConnection;
use tokio::sync::RwLock;

use crate::{
    core::bindings::{
        BindingsImplState,
        executor::{ErasedExecutor, QueryOrRaw},
        transaction::{ConnectionBoundTask, TransactionCommand},
        wasmledger::sql::{connection::Connection, transaction::Transaction, util_types::Error},
    },
    execute_with,
    sqldb::SqlDatabase,
};

use crate::core::bindings::transaction::TransactionImpl;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ConnectionImpl {
    pub(crate) connection: Arc<RwLock<PoolConnection<SqlDatabase>>>,
}

impl crate::core::bindings::wasmledger::sql::connection::Host for BindingsImplState {}

impl crate::core::bindings::wasmledger::sql::connection::HostConnection for BindingsImplState {
    async fn drop(
        &mut self,
        rep: wasmtime::component::Resource<Connection>,
    ) -> wasmtime::Result<()> {
        self.table.delete(rep)?;

        Ok(())
    }
}

impl crate::core::bindings::wasmledger::sql::connection::HostConnectionWithStore
    for BindingsImplState
{
    async fn begin_transaction<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        self_: wasmtime::component::Resource<Connection>,
    ) -> Result<wasmtime::component::Resource<Transaction>, Error> {
        let (sender, receiver) = tokio::sync::mpsc::channel::<TransactionCommand>(1);
        let handle = accessor.spawn(ConnectionBoundTask {
            resource: self_,
            receiver,
        });

        let tx_impl = TransactionImpl::ConnectionBound {
            handle: Arc::new(handle),
            sender,
        };

        let resource = accessor.with(|mut access| {
            let state = access.get();

            state.table.push(tx_impl)
        })?;

        return Ok(resource);
    }
}

impl ErasedExecutor<BindingsImplState> for ConnectionImpl {
    async fn fetch_all<T>(
        &self,
        query: QueryOrRaw,
        accessor: &wasmtime::component::Accessor<T, BindingsImplState>,
    ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error> {
        let mut guard = self.connection.write().await;

        execute_with!(guard, accessor, query, fetch_all)
    }

    async fn execute<T>(
        &self,
        query: QueryOrRaw,
        accessor: &wasmtime::component::Accessor<T, BindingsImplState>,
    ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error> {
        let mut guard = self.connection.write().await;

        execute_with!(guard, accessor, query, execute)
    }
}
