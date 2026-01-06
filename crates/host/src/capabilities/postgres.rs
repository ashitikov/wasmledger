use std::{env, str::FromStr};

use wasmledger_sql::sqldb::{SqlDB, sqlx};
use wasmtime::component::Linker;

use crate::engine::CoreState;

pub type PostgresState = wasmledger_sql::core::bindings::BindingsImplState;

pub(crate) async fn create_postgres_state() -> anyhow::Result<PostgresState> {
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

    Ok(PostgresState::new(SqlDB::new(pool)))
}

pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
    wasmledger_sql::core::bindings::Host_::add_to_linker::<CoreState, PostgresState>(
        linker,
        |s| &mut s.postgres,
    )?;

    wasmledger_sql::postgres::bindings::CodecsPostgres::add_to_linker::<CoreState, PostgresState>(
        linker,
        |s| &mut s.postgres,
    )?;

    Ok(())
}
