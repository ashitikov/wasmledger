use wasm_sql::sqldb::sqlx;

use crate::capabilities::postgres::{self, PgPool};
use crate::extension::client::ExtensionClient;

pub async fn apply_from_extensions(client: &ExtensionClient) -> anyhow::Result<()> {
    let pool = postgres::get_pool();
    ensure_migrations_table(pool).await?;

    let all_migrations = client.get_migrations().await?;
    for ext_mig in &all_migrations {
        apply_pending(pool, &ext_mig.id, &ext_mig.migrations).await?;
    }

    Ok(())
}

async fn ensure_migrations_table(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _migrations (
            extension_id TEXT NOT NULL,
            migration_id TEXT NOT NULL,
            applied_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
            PRIMARY KEY (extension_id, migration_id)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Apply pending migrations from the provided list of (id, sql) pairs.
/// Migrations already recorded in _migrations are skipped.
async fn apply_pending(
    pool: &PgPool,
    group: &str,
    migrations: &[(String, String)],
) -> anyhow::Result<()> {
    if migrations.is_empty() {
        return Ok(());
    }

    let applied = list_applied(pool, group).await?;

    let pending: Vec<_> = migrations
        .iter()
        .filter(|(id, _)| !applied.contains(id))
        .collect();

    if pending.is_empty() {
        println!("  No pending migrations for '{group}'");
        return Ok(());
    }

    println!(
        "  Applying {} migration(s) for '{group}'...",
        pending.len()
    );

    for (id, sql) in &pending {
        let mut tx = pool.begin().await?;

        sqlx::raw_sql(sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| anyhow::anyhow!("Migration '{id}' failed for '{group}': {e}"))?;

        sqlx::query("INSERT INTO _migrations (extension_id, migration_id) VALUES ($1, $2)")
            .bind(group)
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        println!("    Applied: {id}");
    }

    Ok(())
}

async fn list_applied(pool: &PgPool, group: &str) -> anyhow::Result<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT migration_id FROM _migrations WHERE extension_id = $1 ORDER BY migration_id",
    )
    .bind(group)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(id,)| id).collect())
}
