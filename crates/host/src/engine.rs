use wasmtime::{Engine, Store, component::Linker};

use crate::capabilities::sql::create_sql_state;

struct GlobalComponentState {
    sql: wasmledger_sql::CoreComponentState,
}

pub(crate) async fn create_wasm_engine() -> anyhow::Result<Engine> {
    let engine = Engine::default();

    let store_state = create_store_state().await?;

    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, store_state);

    add_to_linker(&mut linker);

    anyhow::Result::Ok(engine)
}

async fn create_store_state() -> anyhow::Result<GlobalComponentState> {
    let sql = create_sql_state().await?;

    let state = GlobalComponentState { sql };

    anyhow::Result::Ok(state)
}

fn add_to_linker(linker: &mut Linker<GlobalComponentState>) {
    let _ = wasmledger_sql::CoreHost::add_to_linker::<
        GlobalComponentState,
        wasmledger_sql::CoreComponentState,
    >(linker, |s| &mut s.sql);
}
