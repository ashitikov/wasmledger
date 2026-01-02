use std::{env, str::FromStr};

use wasmledger_sql::{
    CoreComponentState,
    sqldb::{SqlDB, sqlx},
};

pub(crate) async fn create_sql_state() -> anyhow::Result<CoreComponentState> {
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

    Ok(CoreComponentState::new(SqlDB::new(pool)))
}
