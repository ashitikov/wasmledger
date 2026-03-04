use wasmledger_sql_client::base::{
    bindings::wasm_sql::core::{
        query::{self, QueryExecutor},
        query_types::{SqlArguments, SqlQuery},
    },
    decode::DecodeRequired,
};
use wasmledger_sql_client::postgres::bindings::wasm_sql::postgres::codecs::{
    get_int64, push_int64,
};

use crate::BindingsImpl;
use crate::bindings::exports::wasmledger::extension::id_gen::Guest;

/// Advisory lock key, pre-computed hash of "next_transfer_ids"
const LOCK_KEY: i64 = 0x6e78745f7472616e;

impl Guest for BindingsImpl {
    async fn generate_ids(executor: QueryExecutor<'_>, count: u32) -> Result<Vec<i64>, String> {
        let args = SqlArguments::new();
        push_int64(Some(count as i64), &args)?;
        push_int64(Some(LOCK_KEY), &args)?;

        let query = SqlQuery {
            sql: "SELECT * FROM next_transfer_ids($1, $2)".to_string(),
            args: Some(args),
            persistent: Some(true),
        };

        let results = query::fetch_all(query, executor).await?;

        let row = results
            .into_iter()
            .next()
            .ok_or("next_transfer_ids returned no rows".to_string())?;

        let seq_start = row.column("seq_start").decode(get_int64).required()?;
        let seq_end = row.column("seq_end").decode(get_int64).required()?;

        Ok((seq_start..=seq_end).collect())
    }
}
