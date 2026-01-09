use tokio::sync::{mpsc::error::SendError, oneshot::error::RecvError};
use wasmtime::component::ResourceTableError;

use crate::core::bindings::wasmledger::sql::util_types::Error;

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolTimedOut => Error::PoolTimedOut,
            sqlx::Error::PoolClosed => Error::PoolClosed,
            sqlx::Error::Decode(_) => Error::Decode(err.to_string()),
            sqlx::Error::Encode(_) => Error::Encode(err.to_string()),
            sqlx::Error::BeginFailed => Error::BeginFailed,
            err => Error::Unexpected(err.to_string()),
        }
    }
}

impl From<ResourceTableError> for Error {
    fn from(value: ResourceTableError) -> Self {
        Self::Unexpected(value.to_string())
    }
}

impl From<RecvError> for Error {
    fn from(value: RecvError) -> Self {
        Self::Unexpected(value.to_string())
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        Self::Unexpected(value.to_string())
    }
}