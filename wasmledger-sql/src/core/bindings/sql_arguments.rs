use crate::sqldb::SqlDatabase;
use tokio::sync::RwLock;

pub struct SqlArgumentsImpl<'q> {
    pub(crate) args: RwLock<<SqlDatabase as sqlx::Database>::Arguments<'q>>,
}

impl crate::core::bindings::exports::wasmledger::sql::query_types::GuestSqlArguments
    for SqlArgumentsImpl<'static>
{
    fn new() -> Self {
        Self {
            args: RwLock::new(<SqlDatabase as sqlx::Database>::Arguments::default()),
        }
    }
}
