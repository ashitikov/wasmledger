mod lifecycle;
mod migration;

use std::sync::{Arc, Mutex};

use anyhow::Result;

pub use migration::ExtensionMigrations;

pub(crate) use lifecycle::LifecycleClient;
pub(crate) use migration::MigrationClient;

// --- ExtensionClientBuilder ---

#[derive(Clone)]
pub(crate) struct ExtensionClientBuilder {
    inner: Arc<Mutex<ExtensionClientBuilderInner>>,
}

struct ExtensionClientBuilderInner {
    lifecycle_clients: Vec<(String, LifecycleClient)>,
    migration_clients: Vec<(String, MigrationClient)>,
}

impl ExtensionClientBuilder {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ExtensionClientBuilderInner {
                lifecycle_clients: Vec::new(),
                migration_clients: Vec::new(),
            })),
        }
    }

    pub fn add(
        &self,
        extension_id: String,
        lifecycle: Option<LifecycleClient>,
        migration: Option<MigrationClient>,
    ) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(lc) = lifecycle {
            inner.lifecycle_clients.push((extension_id.clone(), lc));
        }
        if let Some(mig) = migration {
            inner.migration_clients.push((extension_id, mig));
        }
    }

    pub fn build(self) -> ExtensionClient {
        let inner = match Arc::try_unwrap(self.inner) {
            Ok(mutex) => mutex.into_inner().unwrap(),
            Err(_) => unreachable!("all shared references consumed before build()"),
        };
        ExtensionClient {
            lifecycle_clients: inner.lifecycle_clients,
            migration_clients: inner.migration_clients,
        }
    }
}

// --- ExtensionClient ---

pub struct ExtensionClient {
    lifecycle_clients: Vec<(String, LifecycleClient)>,
    migration_clients: Vec<(String, MigrationClient)>,
}

impl ExtensionClient {
    /// Call on-init on all extensions sequentially. Fails on first error.
    pub async fn on_init(&self) -> Result<()> {
        for (id, client) in &self.lifecycle_clients {
            println!("  Calling on-init for '{id}'...");
            client
                .on_init()
                .await
                .map_err(|e| anyhow::anyhow!("on-init failed for '{id}': {e}"))?;
        }
        Ok(())
    }

    /// Call on-shutdown on all extensions sequentially. Fails on first error.
    pub async fn on_shutdown(&self) -> Result<()> {
        for (id, client) in &self.lifecycle_clients {
            println!("  Calling on-shutdown for '{id}'...");
            client
                .on_shutdown()
                .await
                .map_err(|e| anyhow::anyhow!("on-shutdown failed for '{id}': {e}"))?;
        }
        Ok(())
    }

    /// Gather migrations from all extensions sequentially.
    /// Returns one entry per extension that exports the migration interface.
    pub async fn get_migrations(&self) -> Result<Vec<ExtensionMigrations>> {
        let mut result = Vec::new();

        for (id, client) in &self.migration_clients {
            let ext_id = client
                .extension_id()
                .await
                .map_err(|e| anyhow::anyhow!("extension-id failed for '{id}': {e}"))?;

            let migrations = client
                .get_migrations()
                .await
                .map_err(|e| anyhow::anyhow!("get-migrations failed for '{id}': {e}"))?;

            result.push(ExtensionMigrations {
                id: ext_id,
                migrations,
            });
        }

        Ok(result)
    }
}
