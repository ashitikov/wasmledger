// Execution module re-exports

pub mod error;
pub mod executor;

pub use error::ExecutionError;
pub use executor::{FunctionExecutor, ExecutionResult};
