use crate::{
    core::bindings::{BindingsImplState, wasmledger::sql::query_types::QueryResults},
    sqldb::SqlDatabase,
};

#[allow(unused)]
pub struct QueryResultsImpl {
    pub(crate) results: Vec<<SqlDatabase as sqlx::Database>::Row>,
}

impl crate::core::bindings::wasmledger::sql::query_types::HostQueryResults for BindingsImplState {
    fn row_count(
        &mut self,
        self_: wasmtime::component::Resource<QueryResults>,
    ) -> Result<u64, wasmtime::Error> {
        let a = self.table.get(&self_)?;

        Ok(a.results.len() as u64)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<QueryResults>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;

        Ok(())
    }
}
