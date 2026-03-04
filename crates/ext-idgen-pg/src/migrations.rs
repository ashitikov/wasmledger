use wasmledger_utils::sql_migration;

use crate::BindingsImpl;
use crate::bindings::exports::wasmledger::extension::migration::{Guest, Migration};

impl Guest for BindingsImpl {
    fn extension_id() -> String {
        "wasmledger_idgen_pg".to_string()
    }

    fn get_migrations() -> Vec<Migration> {
        vec![sql_migration!("001_idgen")]
    }
}
