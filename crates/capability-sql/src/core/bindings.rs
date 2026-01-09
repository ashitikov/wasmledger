use std::sync::Arc;

use wasmtime::component::{HasData, ResourceTable};

use crate::sqldb::SqlDB;

wasmtime::component::bindgen!({
    path: ["../../wit/sql", "./wit"],
    world: "wasmledger:sql-host/host",
    imports: {
        "wasmledger:sql/pool": async,
        "wasmledger:sql/transaction": async,
        "wasmledger:sql/connection": async,
        "wasmledger:sql/query-types": trappable,
    },
    with: {
        "wasmledger:sql/query-types.query-results": crate::core::bindings::query_results::QueryResultsImpl,
        "wasmledger:sql/query-types.sql-arguments": crate::core::bindings::sql_arguments::SqlArgumentsImpl,
        "wasmledger:sql/transaction.transaction": crate::core::bindings::transaction::TransactionImpl,
        "wasmledger:sql/connection.connection": crate::core::bindings::connection::ConnectionImpl,
    }
});

#[allow(unused)]
pub struct BindingsImplState {
    pub(crate) table: ResourceTable,
    pub(crate) sql_db: Arc<SqlDB>,
}

impl BindingsImplState {
    pub fn new(db: Arc<SqlDB>) -> Self {
        Self {
            table: ResourceTable::new(),
            sql_db: db,
        }
    }
}

impl HasData for BindingsImplState {
    type Data<'a>
        = &'a mut BindingsImplState
    where
        Self: 'a;
}

impl crate::core::bindings::wasmledger::sql::util_types::Host for BindingsImplState {}

pub(crate) mod connection;
pub(crate) mod error;
pub(crate) mod executor;
pub(crate) mod pool;
pub(crate) mod query;
pub(crate) mod query_results;
pub(crate) mod sql_arguments;
pub(crate) mod transaction;
