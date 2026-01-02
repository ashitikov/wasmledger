use sqlx::{Pool};

#[cfg(feature = "postgres")]
pub(crate) type SqlDatabase = sqlx::Postgres;

#[cfg(feature = "mysql")]
pub(crate) type SqlDatabase = sqlx::Mysql;

#[cfg(any(feature = "sqlite", feature = "sqlite-unbundled"))]
pub(crate) type SqlDatabase = sqlx::Sqlite;

pub struct SqlDB {
    pub(crate) pool: Pool<SqlDatabase>,
}

impl SqlDB {
    pub fn new(pool: Pool<SqlDatabase>) -> Self {
        Self { pool: pool }
    }
} 

// pub use sqlx::pool::PoolOptions as PoolOptions;
pub type SqlDatabasePoolOption = sqlx::pool::PoolOptions<SqlDatabase>;
