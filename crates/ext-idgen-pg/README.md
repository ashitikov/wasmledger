# ext-idgen-pg

PostgreSQL-based monotonic ID generator for wasmledger transfers.

## ID structure

```
 63                              10  9          0
 ┌──────────────────────────────────┬────────────┐
 │         base (53 bits)           │ shard (10) │
 └──────────────────────────────────┴────────────┘
```

- **base** — approximate epoch microseconds (with small drift under load)
- **shard** — database shard identifier (0..1023)

## Extraction

```sql
-- shard
SELECT id & x'3FF'::bigint AS shard_id;

-- approximate timestamp (unix seconds)
SELECT to_timestamp((id >> 10) / 1000000.0 + epoch_offset) AS created_at;
```

```rust
let shard_id = id & 0x3FF;
let timestamp_sec = (id >> 10) / 1_000_000 + epoch_offset;
```

## Guarantees

- **Strictly monotonic within a shard** — advisory lock serializes all calls to `next_id_bases`; `greatest(last_value + 1, clock_μs)` ensures the base never decreases.
- **Clock jump resilient** — backward jump: sequence value protects against regression; forward jump: base catches up to real time.
- **Batch support** — single advisory lock acquisition per batch; returns a contiguous range of bases.

## Drift

Each batch of `count` IDs advances the base by `count` microseconds. At 20K transfers/sec with batch size 100, drift is ~100μs — self-recovers during any pause between calls.

## Capacity

With 10 shard bits, the 53-bit base space provides ~230 years from Unix epoch (overflow around year ~2255). With a custom `epoch_offset` (e.g., 2025-01-01), the full ~285 years are available.

## PG function

`next_id_bases(count, lock_key, epoch_offset DEFAULT 0)` — returns `(seq_start, seq_end)` base range. Uses `pg_advisory_lock` for concurrency safety. The shard embedding (`(base << 10) | shard_id`) is performed in the WASM extension, not in PostgreSQL.

## Cross-shard ordering

Not guaranteed. Two shards generating IDs simultaneously will produce interleaved values. Ordering is only guaranteed **within** a single shard.
