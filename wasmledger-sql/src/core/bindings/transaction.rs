use tokio::sync::RwLock;

use crate::{
    core::bindings::{
        BindingsImpl, exports::wasmledger::sql::transaction::Transaction,
        wasmledger::sql::util_types::Error,
    },
    sqldb::SqlDatabase,
};

pub struct TransactionImpl<'c> {
    pub(crate) tx: RwLock<sqlx::Transaction<'c, SqlDatabase>>,
}

impl crate::core::bindings::exports::wasmledger::sql::transaction::Guest for BindingsImpl {
    type Transaction = TransactionImpl<'static>;
}

impl crate::core::bindings::exports::wasmledger::sql::transaction::GuestTransaction
    for TransactionImpl<'static>
{
    async fn commit(this: Transaction) -> Result<(), Error> {
        let tx_impl: TransactionImpl = this.into_inner();
        let tx = tx_impl.tx.into_inner();

        tx.commit().await?;
        Ok(())
    }

    async fn rollback(this: Transaction) -> Result<(), Error> {
        let tx_impl: TransactionImpl = this.into_inner();
        let tx = tx_impl.tx.into_inner();

        tx.rollback().await?;
        Ok(())
    }
}
