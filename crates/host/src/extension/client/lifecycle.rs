use anyhow::Result;
use wasm_runtime_composer::ExportFunc;

pub(crate) struct LifecycleClient {
    on_init: ExportFunc,
    on_shutdown: ExportFunc,
}

impl LifecycleClient {
    pub fn new(on_init: ExportFunc, on_shutdown: ExportFunc) -> Self {
        Self { on_init, on_shutdown }
    }

    pub async fn on_init(&self) -> Result<()> {
        self.on_init.call(&[], &mut []).await?;
        Ok(())
    }

    pub async fn on_shutdown(&self) -> Result<()> {
        self.on_shutdown.call(&[], &mut []).await?;
        Ok(())
    }
}
