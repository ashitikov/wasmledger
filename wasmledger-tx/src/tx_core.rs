use crate::TxCore;
use crate::bindings;
use crate::bindings::wasmledger::tx::types::Transfer;
use crate::bindings::wasmledger::tx::types::{WalletError, WalletRef};

impl bindings::exports::wasmledger::tx::core::Guest for TxCore {
    fn acquire_wallet(id: String) -> Result<WalletRef, WalletError> {
        todo!()
    }

    fn commit_transfers(transfers: Vec<Transfer>) -> () {
        todo!()
    }
}
