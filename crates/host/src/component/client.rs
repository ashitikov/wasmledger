use wasmtime::component::{Linker};

use crate::engine::CoreState;

pub(crate) mod migrator;
pub(crate) mod component;

/// Trait for plugin interfaces that can be linked by the host
pub trait PluginClient {
    fn is_interface_supported(interface: &str) -> bool;

    /// Try to add this interface to the linker if component supports it
    /// Returns Ok(()) if added, Err if not supported
    fn add_to_linker(
        linker: &mut Linker<CoreState>,
    ) -> anyhow::Result<()>;
}

pub fn add_all_clients_to_linker(
    linker: &mut Linker<CoreState>,
) -> anyhow::Result<()> {

    migrator::MigrationsPluginClient::add_to_linker(linker)?;

    Ok(())
}
