mod bindings {
    wit_bindgen::generate!({
        path: ["../../wit/money", "../../wit/tx"],
        world: "wasmledger:tx/tx",
        with: { "wasmledger:money/types": generate }
    });

    use super::TxCore;
    export!(TxCore);
}

mod tx_core;

pub struct TxCore;
