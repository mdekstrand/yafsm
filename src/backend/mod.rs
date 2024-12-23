//! Backend data collector implementations.
use std::time::Duration;

use crate::model::*;

pub mod error;
#[cfg(target_os = "linux")]
pub mod linux;
pub mod sysinfo;
pub mod util;

pub use error::{BackendError, BackendResult};

/// Trait implemented by backend implementations.
#[allow(dead_code)]
pub trait MonitorBackend {
    /// Refresh the system status data.
    fn update(&mut self, opts: &Options) -> BackendResult<()>;

    /// Get the hostname
    fn hostname(&self) -> BackendResult<String>;

    /// Get the kernel
    fn system_version(&self) -> BackendResult<String>;

    /// Get the system uptime.
    fn uptime(&self) -> BackendResult<Duration>;

    /// Get the number of physical CPU cores.
    fn cpu_count(&self) -> BackendResult<u32>;

    /// Get the number of physical CPU cores.
    fn logical_cpu_count(&self) -> BackendResult<u32>;

    /// Get overall CPU utilization.
    fn global_cpu(&self) -> BackendResult<CPU>;

    /// Get memory usage.
    fn memory(&self) -> BackendResult<Memory>;

    /// Get swap usage.
    fn swap(&self) -> BackendResult<Swap>;

    /// Get the system load average.
    fn load_avg(&self) -> BackendResult<LoadAvg>;

    /// Get system pressure info.
    fn pressure(&self) -> BackendResult<SystemPressure> {
        Err(BackendError::NotSupported)
    }

    /// Get the running processes.
    fn processes<'a>(&'a self) -> BackendResult<Vec<Process>> {
        Err(BackendError::NotSupported)
    }

    /// Get the comamnd information for a process.
    fn process_cmd_info(&self, _pid: u32) -> BackendResult<ProcessCommandInfo> {
        Err(BackendError::NotSupported)
    }

    /// Get the networks.
    fn networks(&self) -> BackendResult<Vec<NetworkStats>> {
        Err(BackendError::NotSupported)
    }

    /// Get the disks.
    fn disks(&self) -> BackendResult<Vec<DiskIO>> {
        Err(BackendError::NotSupported)
    }

    /// Get the filesystems.
    fn filesystems(&self) -> BackendResult<Vec<Filesystem>> {
        Err(BackendError::NotSupported)
    }

    /// Get the GPUs.
    fn gpus(&self) -> BackendResult<Vec<GPUStats>> {
        Err(BackendError::NotAvailable)
    }

    fn has_process_time(&self) -> bool {
        false
    }

    fn has_gpus(&mut self) -> bool {
        false
    }
}
