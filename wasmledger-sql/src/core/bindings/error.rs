use crate::core::bindings::wasmledger::sql::util_types::Error;

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolTimedOut => Error::PoolTimedOut,
            sqlx::Error::PoolClosed => Error::PoolClosed,
            sqlx::Error::Decode(_) => Error::Decode(err.to_string()),
            sqlx::Error::Encode(_) => Error::Encode(err.to_string()),
            sqlx::Error::BeginFailed => Error::BeginFailed,
            sqlx::Error::RowNotFound => Error::RowNotFound,
            err => Error::Unexpected(err.to_string()),
        }
    }
}
