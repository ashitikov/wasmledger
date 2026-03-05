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

use crate::bindings::exports::wasmledger::extension::id_gen::Guest;
use crate::{BindingsImpl, bindings::exports::wasmledger::extension::id_gen::GenIdContext};

/// ID structure (64 bits):
///   [1 sign][53 base (≈epoch μs)][10 shard]
///
/// Extraction:
///   shard_id  = id & 0x3FF
///   timestamp = (id >> 10) / 1_000_000 + epoch_offset  (unix seconds, approximate)
///
/// Guarantees:
///   - Strictly monotonic within a single shard (advisory lock + greatest)
///   - Resilient to clock jumps (backward: sequence protects, forward: catches up)
///   - Batch drift: ~count μs per call, self-recovers during pauses

/// Advisory lock key, pre-computed hash of "next_id_bases"
const LOCK_KEY: i64 = 0x6e78745f7472616e;
const SHARD_BITS: u32 = 10;
const MAX_SHARD_ID: u16 = (1 << SHARD_BITS) - 1;

impl Guest for BindingsImpl {
    async fn generate_ids(count: u32, context: GenIdContext<'_>) -> Result<Vec<i64>, String> {
        if context.shard_id > MAX_SHARD_ID {
            return Err(format!(
                "shard_id {} exceeds maximum {}", context.shard_id, MAX_SHARD_ID
            ));
        }

        let shard = context.shard_id as i64;

        let args = SqlArguments::new();
        push_int64(Some(count as i64), &args)?;
        push_int64(Some(LOCK_KEY), &args)?;

        let query = SqlQuery {
            sql: "SELECT * FROM next_id_bases($1, $2)".to_string(),
            args: Some(args),
            persistent: Some(true),
        };

        let results = query::fetch_all(query, context.executor).await?;

        let row = results
            .into_iter()
            .next()
            .ok_or("next_id_bases returned no rows".to_string())?;

        let seq_start = row.column("seq_start").decode(get_int64).required()?;
        let seq_end = row.column("seq_end").decode(get_int64).required()?;

        Ok((seq_start..=seq_end)
            .map(|base| (base << SHARD_BITS) | shard)
            .collect())
    }
}
