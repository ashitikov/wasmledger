use std::{env, str::FromStr, sync::Arc};

use tokio::sync::OnceCell;
use wasmledger_capability_sql::sqldb::{SqlDB, sqlx};
use wasmtime::component::Linker;

use crate::engine::CoreState;

pub type PostgresState = wasmledger_capability_sql::core::bindings::BindingsImplState;

static DATABASE: OnceCell<Arc<SqlDB>> = OnceCell::const_new();

pub(crate) async fn create_postgres_state() -> anyhow::Result<PostgresState> {
    let database = DATABASE
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

            Ok(Arc::new(SqlDB::new(pool)))
        })
        .await?;

    Ok(PostgresState::new(database.clone()))
}

pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
    wasmledger_capability_sql::core::bindings::Host_::add_to_linker::<CoreState, PostgresState>(
        linker,
        |s| &mut s.postgres,
    )?;

    wasmledger_capability_sql::postgres::bindings::CodecsPostgres::add_to_linker::<
        CoreState,
        PostgresState,
    >(linker, |s| &mut s.postgres)?;

    Ok(())
}
