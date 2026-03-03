use anyhow::Result;
use wasm_runtime_composer::ExportFunc;
use wasmtime::component::Val;

pub struct ExtensionMigrations {
    pub id: String,
    pub migrations: Vec<(String, String)>,
}

pub(crate) struct MigrationClient {
    extension_id_func: ExportFunc,
    get_migrations_func: ExportFunc,
}

impl MigrationClient {
    pub fn new(extension_id_func: ExportFunc, get_migrations_func: ExportFunc) -> Self {
        Self {
            extension_id_func,
            get_migrations_func,
        }
    }

    pub async fn extension_id(&self) -> Result<String> {
        let mut results = vec![Val::Bool(false)];
        self.extension_id_func.call(&[], &mut results).await?;
        match results.into_iter().next() {
            Some(Val::String(s)) => Ok(s),
            other => anyhow::bail!("unexpected result from extension-id: {other:?}"),
        }
    }

    // Ugly right now, because at the moment of writing wasm-runtime-composer does not have binding generator
    // Since we don't plan having many of such functions, there is no demand right now to invest time to properly generate bindings
    pub async fn get_migrations(&self) -> Result<Vec<(String, String)>> {
        let mut results = vec![Val::Bool(false)];
        self.get_migrations_func.call(&[], &mut results).await?;

        match results.into_iter().next() {
            Some(Val::List(items)) => items
                .into_iter()
                .map(|v| match v {
                    Val::Record(fields) => {
                        let mut id = None;
                        let mut sql = None;
                        for (name, val) in fields {
                            match (name.as_str(), val) {
                                ("id", Val::String(s)) => id = Some(s),
                                ("sql", Val::String(s)) => sql = Some(s),
                                _ => {}
                            }
                        }
                        match (id, sql) {
                            (Some(id), Some(sql)) => Ok((id, sql)),
                            _ => anyhow::bail!("invalid migration record"),
                        }
                    }
                    other => anyhow::bail!("unexpected migration value: {other:?}"),
                })
                .collect(),
            other => anyhow::bail!("unexpected result from get-migrations: {other:?}"),
        }
    }
}
