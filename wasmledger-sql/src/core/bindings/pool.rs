use tokio::sync::RwLock;

use crate::core::bindings::{
    BindingsImpl, context::BindingsContext, exports::wasmledger::sql::pool::Transaction,
    transaction::TransactionImpl, wasmledger::sql::util_types::Error,
};

impl crate::core::bindings::exports::wasmledger::sql::pool::Guest for BindingsImpl {
    async fn begin_transaction() -> Result<Transaction, Error> {
        let pool = BindingsContext::get_pool();
        let tx = pool.begin().await?;

        Ok(Transaction::new(TransactionImpl {
            tx: RwLock::new(tx),
        }))
    }
}
