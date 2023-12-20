//! Backend errors.
use std::io::{self, ErrorKind};

#[cfg(target_os = "linux")]
use nix::errno::Errno;
#[cfg(target_os = "linux")]
use nvml_wrapper::error::NvmlError;
#[cfg(target_os = "linux")]
use procfs::ProcError;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum BackendError {
    #[error("operation not supported")]
    NotSupported,

    #[error("operation not currently available")]
    NotAvailable,

    #[error("item not found")]
    NotFound,

    #[error("not allowed")]
    NotAllowed,

    #[error("IO error: {0}")]
    IOError(io::ErrorKind),

    #[cfg(target_os = "linux")]
    #[error("unix error: {0}")]
    NixError(nix::errno::Errno),

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
    /// - [BackendError::NotAvailable]
    /// - [BackendError::NotFound]
    /// - [BackendError::NotAllowed]
    fn acceptable_to_opt(self) -> BackendResult<Option<T>>;
}

impl<T> BackendErrorFilter<T> for BackendResult<T> {
    fn acceptable_to_opt(self) -> BackendResult<Option<T>> {
        match self {
            Ok(r) => Ok(Some(r)),
            Err(
                BackendError::NotSupported
                | BackendError::NotFound
                | BackendError::NotAllowed
                | BackendError::NotAvailable,
            ) => Ok(None),
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
impl From<Errno> for BackendError {
    fn from(err: Errno) -> Self {
        match err {
            Errno::EPERM | Errno::EACCES => Self::NotAllowed,
            Errno::ENOENT => Self::NotFound,
            k => Self::NixError(k),
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

#[cfg(target_os = "linux")]
impl From<NvmlError> for BackendError {
    fn from(err: NvmlError) -> Self {
        use NvmlError::*;
        match err {
            NoPermission => BackendError::NotAllowed,
            NotFound | GpuLost => BackendError::NotFound,
            DriverNotLoaded => BackendError::NotAvailable,
            NotSupported => BackendError::NotSupported,
            _ => generic_err(format!("NVML error: {}", err)),
        }
    }
}

/// Create a generic backend error.
pub(super) fn generic_err<S: AsRef<str>>(msg: S) -> BackendError {
    BackendError::Other(msg.as_ref().to_string())
}
