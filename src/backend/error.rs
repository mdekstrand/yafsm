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

pub trait BackendErrorFilter<T> {
    /// Convert “acceptable” errors (missing/inaccessible data, as opposed to
    /// system errors retrieving data) into an `Ok(None)`.  The acceptable
    /// errors are:
    ///
    /// - [BackendError::NotSupported]
    /// - [BackendError::NotFound]
    /// - [BackendError::NotAllowed]
    fn acceptable_to_opt(self) -> BackendResult<Option<T>>;
}

impl<T> BackendErrorFilter<T> for BackendResult<T> {
    fn acceptable_to_opt(self) -> BackendResult<Option<T>> {
        match self {
            Ok(r) => Ok(Some(r)),
            Err(BackendError::NotSupported | BackendError::NotFound | BackendError::NotAllowed) => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

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
