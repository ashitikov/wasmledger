use crate::core::bindings::wasmledger::sql::query_types::SqlArguments;
use crate::{core::bindings::BindingsImplState, sqldb::SqlDatabase};
use tokio::sync::RwLock;

#[allow(unused)]
pub struct SqlArgumentsImpl {
    pub(crate) args: RwLock<<SqlDatabase as sqlx::Database>::Arguments<'static>>,
}

impl crate::core::bindings::wasmledger::sql::query_types::HostSqlArguments for BindingsImplState {
    fn new(&mut self) -> Result<wasmtime::component::Resource<SqlArgumentsImpl>, wasmtime::Error> {
        let args = self.table.push(SqlArgumentsImpl {
            args: RwLock::new(<SqlDatabase as sqlx::Database>::Arguments::default()),
        })?;

        Ok(args)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<SqlArguments>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;

        Ok(())
    }
}
