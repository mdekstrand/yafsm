//! Backend errors.
use std::io::{self, ErrorKind};

#[cfg(target_os = "linux")]
use procfs::ProcError;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum BackendError {
    #[error("operation not supported")]
    NotSupported,

    #[error("item not found")]
    NotFound,

    #[error("not allowed")]
    NotAllowed,

    #[error("IO error: {0}")]
    IOError(io::ErrorKind),

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
            k => Self::IOError(k),
        }
    }
}

#[cfg(target_os = "linux")]
impl From<ProcError> for BackendError {
    fn from(err: ProcError) -> Self {
        match err {
            ProcError::NotFound(_) => Self::NotFound,
            ProcError::PermissionDenied(_) => Self::NotAllowed,
            ProcError::Io(e, _) => Self::IOError(e.kind()),
            ProcError::Incomplete(Some(p)) => Self::Other(format!("{:?} incomplete", p)),
            ProcError::Incomplete(None) => Self::Other("incomplete file".into()),
            ProcError::InternalError(e) => Self::Other(format!("internal procfs error: {}", e)),
            ProcError::Other(s) => Self::Other(s),
        }
    }
}

#[cfg(target_os = "linux")]
impl From<etc_os_release::Error> for BackendError {
    fn from(err: etc_os_release::Error) -> Self {
        use etc_os_release::Error;
        match err {
            Error::NoOsRelease => BackendError::NotSupported,
            Error::Open { err, .. } => BackendError::IOError(err.kind()),
            Error::Read { err } => BackendError::IOError(err.kind()),
            _ => generic_err("unknown OS release error"),
        }
    }
}

/// Create a generic backend error.
pub(super) fn generic_err<S: AsRef<str>>(msg: S) -> BackendError {
    BackendError::Other(msg.as_ref().to_string())
}
