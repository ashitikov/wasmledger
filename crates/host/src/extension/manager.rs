use std::future::Future;
use std::pin::Pin;

use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasm_runtime_composer::composable::linker_instance_ops::LinkerInstanceOps;
use wasm_runtime_composer::{
    Composable, ComposableComponent, ComposableDescriptor, ComposableType,
    Composer, Composition, CompositionError, ExportFilter,
};

use crate::config::HostConfig;
use crate::extension::client::{ExtensionClient, ExtensionClientBuilder, LifecycleClient, MigrationClient};
use crate::{
    capabilities,
    engine::{CoreState, create_core_state},
};

const LIFECYCLE_INTERFACE: &str = "wasmledger:extension/lifecycle@0.1.0";
const MIGRATION_INTERFACE: &str = "wasmledger:extension/migration@0.1.0";

// --- ComposableExtension: intercepts into_composition() to extract client handles ---

struct ComposableExtension<C: Composable> {
    inner: C,
    builder: ExtensionClientBuilder,
    extension_id: String,
}

impl<C: Composable> ComposableExtension<C> {
    fn new(inner: C, extension_id: String, builder: ExtensionClientBuilder) -> Self {
        Self {
            inner,
            builder,
            extension_id,
        }
    }
}

impl<C: Composable + 'static> Composable for ComposableExtension<C> {
    fn ty(&self) -> &ComposableType {
        self.inner.ty()
    }

    fn link_export(
        &mut self,
        name: &str,
        importer_linker: &mut dyn LinkerInstanceOps,
    ) -> Result<(), CompositionError> {
        self.inner.link_export(name, importer_linker)
    }

    fn link_import(
        &mut self,
        name: &str,
        exporter: &mut dyn Composable,
    ) -> Result<(), CompositionError> {
        self.inner.link_import(name, exporter)
    }

    fn into_composition(
        self: Box<Self>,
    ) -> Pin<Box<dyn Future<Output = Result<Composition, CompositionError>> + Send>> {
        let this = *self;
        Box::pin(async move {
            let composition = Box::new(this.inner).into_composition().await?;

            // Extract lifecycle funcs (optional — extension may not export lifecycle)
            let lifecycle = match composition.get_func(Some(LIFECYCLE_INTERFACE), "on-init") {
                Ok(on_init) => {
                    let on_shutdown = composition
                        .get_func(Some(LIFECYCLE_INTERFACE), "on-shutdown")
                        .map_err(|_| {
                            CompositionError::LinkingError(format!(
                                "Extension '{}' exports on-init but not on-shutdown",
                                this.extension_id
                            ))
                        })?;
                    Some(LifecycleClient::new(on_init, on_shutdown))
                }
                Err(_) => None,
            };

            // Extract migration funcs (optional — extension may not export migration)
            let migration =
                match composition.get_func(Some(MIGRATION_INTERFACE), "extension-id") {
                    Ok(extension_id_func) => {
                        let get_migrations = composition
                            .get_func(Some(MIGRATION_INTERFACE), "get-migrations")
                            .map_err(|_| {
                                CompositionError::LinkingError(format!(
                                    "Extension '{}' exports extension-id but not get-migrations",
                                    this.extension_id
                                ))
                            })?;
                        Some(MigrationClient::new(extension_id_func, get_migrations))
                    }
                    Err(_) => None,
                };

            this.builder.add(this.extension_id, lifecycle, migration);

            Ok(composition)
        })
    }
}

// --- ExtensionManager ---

pub struct ExtensionManager {
    composition: Composition,
    client: ExtensionClient,
    extension_ids: Vec<String>,
}

impl ExtensionManager {
    pub async fn load_from_config(engine: &Engine, config: &HostConfig) -> anyhow::Result<Self> {
        let mut composer = Composer::new();
        let mut extension_ids = Vec::new();
        let builder = ExtensionClientBuilder::new();

        for entry in &config.extensions {
            let component = Component::from_file(engine, &entry.path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to compile extension '{}' from {:?}: {}",
                    entry.id,
                    entry.path,
                    e
                )
            })?;

            let linker = create_linker(engine)?;
            let engine_clone = engine.clone();

            let composable = ComposableComponent::new(
                component,
                linker,
                move || {
                    let state = create_core_state();
                    Store::new(&engine_clone, state)
                },
            );

            let extension = ComposableExtension::new(composable, entry.id.clone(), builder.clone());
            let filtered = extension.hiding(&[LIFECYCLE_INTERFACE, MIGRATION_INTERFACE]);

            let descriptor = ComposableDescriptor::new(&entry.id, filtered);
            composer.add(descriptor);
            extension_ids.push(entry.id.clone());

            tracing::debug!(extension = %entry.id, "Extension added to composer");
        }

        let composition = composer
            .compose()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to compose extensions: {}", e))?;

        let client = builder.build();

        Ok(Self {
            composition,
            client,
            extension_ids,
        })
    }

    pub fn composition(&self) -> &Composition {
        &self.composition
    }

    pub fn client(&self) -> &ExtensionClient {
        &self.client
    }

    pub fn extension_ids(&self) -> &[String] {
        &self.extension_ids
    }

    pub fn count(&self) -> usize {
        self.extension_ids.len()
    }
}

fn create_linker(engine: &Engine) -> anyhow::Result<Linker<CoreState>> {
    let mut linker = Linker::new(engine);

    capabilities::add_to_linker(&mut linker)?;

    Ok(linker)
}
