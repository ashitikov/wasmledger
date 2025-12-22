use sqlx::{Pool};

use crate::sqldb::{SqlDB, SqlDatabase};
use std::{sync::OnceLock};

pub struct BindingsContext;

static CTX: OnceLock<SqlDB> = OnceLock::new();

impl BindingsContext {
    pub fn get_pool() -> &'static Pool<SqlDatabase> {
        &CTX.get().unwrap().pool
    }

    pub fn init(db: SqlDB) {
        CTX.set(db);
    }
}
