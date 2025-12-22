use crate::sqldb::SqlDatabase;

pub(crate) struct QueryResultsImpl {
    pub(crate) results: Vec<<SqlDatabase as sqlx::Database>::Row>,
}

impl crate::core::bindings::exports::wasmledger::sql::query_types::GuestQueryResults
    for QueryResultsImpl
{
}
