//! Backend errors.
use std::io::{self, ErrorKind};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("operation not supported")]
    NotSupported,

    #[error("item not found")]
    NotFound,

    #[error("not allowed")]
    NotAllowed,

    #[error("IO error: {0}")]
    IOError(io::Error),

    #[error("unknown error: {0}")]
    Other(String),
}

pub type BackendResult<T> = Result<T, BackendError>;

impl From<io::Error> for BackendError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::PermissionDenied => Self::NotAllowed,
            ErrorKind::NotFound => Self::NotFound,
            _ => Self::IOError(err),
        }
    }
}

/// Create a generic backend error.
pub(super) fn generic_err<S: AsRef<str>>(msg: S) -> BackendError {
    BackendError::Other(msg.as_ref().to_string())
}
