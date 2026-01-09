use anyhow::{Ok, Result};
use wasmtime::component::Linker;

use crate::{
    capabilities::postgres::PostgresState,
    component::{client::PluginClient, registry::PluginRegistry},
    engine::CoreState,
};

pub(crate) mod bindings {
    wasmtime::component::bindgen!({
        path: ["../../wit/sql", "../../wit/component", "./wit"],
        world: "wasmledger:component-client/migrator",
        with: {
            "wasmledger:sql/query-types": wasmledger_capability_sql::core::bindings::wasmledger::sql::query_types,
            "wasmledger:sql/util-types": wasmledger_capability_sql::core::bindings::wasmledger::sql::util_types,
            "wasmledger:sql/connection": wasmledger_capability_sql::core::bindings::wasmledger::sql::connection,
            "wasmledger:sql/query": wasmledger_capability_sql::core::bindings::wasmledger::sql::query,
            "wasmledger:sql/transaction": wasmledger_capability_sql::core::bindings::wasmledger::sql::transaction,
        },
        require_store_data_send: true,
    });
}

/// Migrations interface implementation
pub struct MigrationsPluginClient;

const SUPPORTED_INTERFACES: [&str; 1] = ["wasmledger:plugin/migrations"];

impl PluginClient for MigrationsPluginClient {
    fn is_interface_supported(interface: &str) -> bool {
        SUPPORTED_INTERFACES.contains(&interface)
    }

    fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
        bindings::Migrator::add_to_linker::<CoreState, PostgresState>(
            linker,
            |state: &mut CoreState| &mut state.postgres,
        )?;

        Ok(())
    }
}

impl MigrationsPluginClient {
    /// Run migrations for all plugins that support the migrations interface
    ///
    /// Execution order: config file order
    /// Failure policy: stop on first error
    /// Logging: info for each plugin, silent skip for non-supporting plugins
    pub async fn run_migrations(registry: &PluginRegistry) -> Result<()> {
        tracing::info!("Starting plugin migrations...");

        for plugin in registry.all() {
            // Lock store mutex to get mutable access
            let mut store = plugin.store.lock().unwrap();

            if let Some(client) = bindings::Migrator::new(&mut *store, &plugin.instance).ok() {
                tracing::info!(plugin = %plugin.id, "Running migrations");
                let migrator = client.wasmledger_component_migrator();

                let res = store
                    .run_concurrent(async |accessor| -> anyhow::Result<()> {
                        // let executor = QueryExecutor::Pool;
                        // let test = migrator.call_check_schema(accessor, executor).await?;
                        // match test {
                        //     std::result::Result::Ok(v) => println!("check schema done"),
                        //     Err(e) => println!("check schema err {}", e),
                        // }
                        // let migrations = migrator.call_list_migrations(accessor).await?;
                        // println!("Running migrations3");
                        // let t = format!("migrations {}", migrations.join(","));
                        // tracing::info!(plugin = %plugin.id, t);

                        // println!("migrations {}", migrations.join(","));

                        Ok(())
                    })
                    .await;

                match res {
                    std::result::Result::Ok(_) => {
                        todo!()
                    }
                    Err(e) => println!("{}", e),
                }

                tracing::info!(plugin = %plugin.id, "Migrations completed successfully");
            }
        }

        Ok(())
    }
}
