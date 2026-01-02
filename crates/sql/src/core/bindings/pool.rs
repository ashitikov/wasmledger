use std::sync::Arc;

use tokio::sync::RwLock;

use crate::core::bindings::{
    BindingsImplState,
    connection::ConnectionImpl,
    transaction::TransactionImpl,
    wasmledger::sql::{pool::Connection, transaction::Transaction, util_types::Error},
};

impl crate::core::bindings::wasmledger::sql::pool::HostWithStore for BindingsImplState {
    async fn begin_transaction<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
    ) -> Result<wasmtime::component::Resource<Transaction>, Error> {
        let pool = accessor.with(|mut access| access.get().sql_db.pool.clone());
        let tx = pool.begin().await?;
        let tx_impl = TransactionImpl::Tx(Arc::new(RwLock::new(Some(tx))));

        let tx_resource = accessor.with(|mut access| {
            let state = access.get();

            state.table.push(tx_impl)
        })?;

        Ok(tx_resource)
    }

    async fn acquire_connection<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
    ) -> Result<wasmtime::component::Resource<Connection>, Error> {
        let pool = accessor.with(|mut access| access.get().sql_db.pool.clone());
        let conn = pool.acquire().await?;

        let conn_impl = ConnectionImpl {
            connection: Arc::new(RwLock::new(conn)),
        };

        let resource = accessor.with(|mut access| {
            let state = access.get();
            state.table.push(conn_impl)
        })?;

        Ok(resource)
    }
}
