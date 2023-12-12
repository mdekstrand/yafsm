//! Backend data collector implementations.
use std::time::Duration;

use crate::model::*;

pub mod error;
pub mod sysinfo;
pub mod util;

pub use error::{BackendError, BackendResult};

/// Trait implemented by backend implementations.
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

    /// Get the running processes.
    fn processes<'a>(&'a self) -> BackendResult<Vec<Process>>;

    /// Get the comamnd information for a process.
    fn process_cmd_info(&self, pid: u32) -> BackendResult<ProcessCommandInfo>;

    /// Get the networks.
    fn networks(&self) -> BackendResult<Vec<NetworkStats>>;

    /// Get the filesystems.
    fn filesystems(&self) -> BackendResult<Vec<Filesystem>>;

    fn has_process_time(&self) -> bool;
}
