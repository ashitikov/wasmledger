use std::{env, str::FromStr, sync::Arc};

use tokio::sync::OnceCell;
use wasm_sql::sqldb::{SqlDB, sqlx};
use wasmtime::component::Linker;

use crate::engine::CoreState;

pub type PostgresState = wasm_sql::SqlHostState;
pub type PgPool = sqlx::postgres::PgPool;

static DATABASE: OnceCell<Arc<SqlDB>> = OnceCell::const_new();
static POOL: OnceCell<PgPool> = OnceCell::const_new();

/// Initialize the database pool (async, call once at startup)
pub async fn initialize_database() -> anyhow::Result<()> {
    DATABASE
        .get_or_try_init(async || -> anyhow::Result<Arc<SqlDB>> {
            let pool_options = sqlx::postgres::PgPoolOptions::default();
            let connect_options = {
                let env_pgurl = env::var("PGURL");
                let opts = match env_pgurl {
                    Ok(url) => sqlx::postgres::PgConnectOptions::from_str(&url)?,
                    Err(env::VarError::NotPresent) => sqlx::postgres::PgConnectOptions::default(),
                    Err(e) => return anyhow::Result::Err(e.into()),
                };
                opts
            };

            let pool = pool_options.connect_with(connect_options).await?;
            POOL.get_or_init(|| async { pool.clone() }).await;
            Ok(Arc::new(SqlDB::new(pool)))
        })
        .await?;

    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    POOL.get().expect("Database not initialized; call initialize_database() first")
}

/// Create postgres state (sync, requires initialize_database() called first)
pub fn create_postgres_state() -> PostgresState {
    let database = DATABASE
        .get()
        .expect("Database not initialized; call initialize_database() first");
    PostgresState::new(database.clone())
}

pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
    wasm_sql::add_to_linker::<CoreState>(linker)?;
    Ok(())
}
