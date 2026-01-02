use sqlx::Acquire;
use std::sync::Arc;

use tokio::sync::{
    RwLock,
    mpsc::{Receiver, Sender},
    oneshot,
};
use wasmtime::component::{Accessor, AccessorTask, JoinHandle};

use crate::{
    core::bindings::{
        BindingsImplState,
        executor::{ErasedExecutor, QueryOrRaw},
        wasmledger::sql::{connection::Connection, transaction::Transaction, util_types::Error},
    },
    execute_with,
    sqldb::SqlDatabase,
};

#[allow(dead_code)]
pub enum TransactionCommand {
    FetchAll {
        query: QueryOrRaw,
        cb: oneshot::Sender<Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error>>,
    },

    Execute {
        query: QueryOrRaw,
        cb: oneshot::Sender<Result<<SqlDatabase as sqlx::Database>::QueryResult, Error>>,
    },
    Commit {
        cb: oneshot::Sender<Result<(), Error>>,
    },

    Rollback {
        cb: oneshot::Sender<Result<(), Error>>,
    },
}

#[allow(dead_code)]
pub struct ConnectionBoundTask {
    pub(crate) resource: wasmtime::component::Resource<Connection>,
    pub(crate) receiver: Receiver<TransactionCommand>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum TransactionImpl {
    Tx(Arc<RwLock<Option<sqlx::Transaction<'static, SqlDatabase>>>>),
    ConnectionBound {
        handle: Arc<JoinHandle>,
        sender: Sender<TransactionCommand>,
    },
}

#[allow(dead_code)]
trait TransactionErasedOps {
    async fn commit(self) -> Result<(), Error>;
    async fn rollback(self) -> Result<(), Error>;
}

impl ErasedExecutor<BindingsImplState> for TransactionImpl {
    async fn fetch_all<T>(
        &self,
        query: QueryOrRaw,
        accessor: &Accessor<T, BindingsImplState>,
    ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error> {
        match self {
            TransactionImpl::Tx(rw_lock) => {
                let mut guard = rw_lock.write().await;
                if let Some(tx) = guard.as_mut() {
                    execute_with!(&mut **tx, accessor, query, fetch_all)
                } else {
                    Err(Error::TransactionClosed)
                }
            }
            TransactionImpl::ConnectionBound { handle: _, sender } => {
                let (s, r) = oneshot::channel();
                sender
                    .send(TransactionCommand::FetchAll { query, cb: s })
                    .await?;
                r.await?
            }
        }
    }

    async fn execute<T>(
        &self,
        query: super::executor::QueryOrRaw,
        accessor: &Accessor<T, BindingsImplState>,
    ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error> {
        match self {
            TransactionImpl::Tx(rw_lock) => {
                let mut guard = rw_lock.write().await;
                if let Some(tx) = guard.as_mut() {
                    execute_with!(&mut **tx, accessor, query, execute)
                } else {
                    Err(Error::TransactionClosed)
                }
            }
            TransactionImpl::ConnectionBound { handle: _, sender } => {
                let (s, r) = oneshot::channel();
                sender
                    .send(TransactionCommand::Execute { query, cb: s })
                    .await?;
                r.await?
            }
        }
    }
}

impl TransactionErasedOps for TransactionImpl {
    async fn commit(self) -> Result<(), Error> {
        match self {
            TransactionImpl::Tx(rw_lock) => {
                let mut guard = rw_lock.write().await;
                if let Some(tx) = guard.take() {
                    Ok(tx.commit().await?)
                } else {
                    Err(Error::TransactionClosed)
                }
            }
            TransactionImpl::ConnectionBound { handle: _, sender } => {
                let (s, r) = oneshot::channel();
                sender.send(TransactionCommand::Commit { cb: s }).await?;

                r.await?
            }
        }
    }

    async fn rollback(self) -> Result<(), Error> {
        match self {
            TransactionImpl::Tx(rw_lock) => {
                let mut guard = rw_lock.write().await;
                if let Some(tx) = guard.take() {
                    Ok(tx.rollback().await?)
                } else {
                    Err(Error::TransactionClosed)
                }
            }
            TransactionImpl::ConnectionBound { handle: _, sender } => {
                let (s, r) = oneshot::channel();
                sender.send(TransactionCommand::Rollback { cb: s }).await?;

                r.await?
            }
        }
    }
}

impl<T> AccessorTask<T, BindingsImplState, Result<(), wasmtime::Error>> for ConnectionBoundTask {
    async fn run(
        mut self,
        accessor: &wasmtime::component::Accessor<T, BindingsImplState>,
    ) -> Result<(), wasmtime::Error> {
        let conn = accessor.with(|mut access| {
            let state = access.get();

            state
                .table
                .get(&self.resource)
                .map(|x| x.connection.clone())
        })?;

        let mut guard = conn.write().await;
        let mut tx = Some(guard.begin().await?);

        while let Some(cmd) = self.receiver.recv().await {
            match cmd {
                TransactionCommand::FetchAll { query, cb } => {
                    let res = if let Some(ref mut tx) = tx {
                        execute_with!(tx, accessor, query, fetch_all)
                    } else {
                        Err(Error::TransactionClosed)
                    };

                    let _ = cb.send(res);
                }
                TransactionCommand::Execute { query, cb } => {
                    let res = if let Some(ref mut tx) = tx {
                        execute_with!(tx, accessor, query, execute)
                    } else {
                        Err(Error::TransactionClosed)
                    };

                    let _ = cb.send(res);
                }

                TransactionCommand::Commit { cb } => {
                    let res = if let Some(tx) = tx.take() {
                        tx.commit().await.map_err(|e| e.into())
                    } else {
                        Err(Error::TransactionClosed)
                    };

                    let _ = cb.send(res);
                }
                TransactionCommand::Rollback { cb } => {
                    let res = if let Some(tx) = tx.take() {
                        tx.rollback().await.map_err(|e| e.into())
                    } else {
                        Err(Error::TransactionClosed)
                    };

                    let _ = cb.send(res);
                }
            }
        }

        Ok(())
    }
}

impl crate::core::bindings::wasmledger::sql::transaction::HostTransaction for BindingsImplState {
    async fn drop(
        &mut self,
        rep: wasmtime::component::Resource<Transaction>,
    ) -> wasmtime::Result<()> {
        self.table.delete(rep)?;

        Ok(())
    }
}

impl crate::core::bindings::wasmledger::sql::transaction::Host for BindingsImplState {}

impl crate::core::bindings::wasmledger::sql::transaction::HostTransactionWithStore
    for BindingsImplState
{
    async fn commit<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        this: wasmtime::component::Resource<Transaction>,
    ) -> Result<(), Error> {
        let tx_impl = accessor.with(|mut access| access.get().table.delete(this))?;

        tx_impl.commit().await
    }

    async fn rollback<T>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        this: wasmtime::component::Resource<Transaction>,
    ) -> Result<(), Error> {
        let tx_impl = accessor.with(|mut access| access.get().table.delete(this))?;

        tx_impl.rollback().await
    }
}
