//! Backend data collector implementations.
use std::time::Duration;

use crate::model::*;

pub mod error;
#[cfg(target_os = "linux")]
pub mod linux;
pub mod sysinfo;
pub mod util;

pub use error::{BackendError, BackendResult};

#[cfg(target_os = "linux")]
pub use linux::LinuxBackend as DefaultBackend;
#[cfg(not(target_os = "linux"))]
pub use sysinfo::SysInfoBackend as DefaultBackend;

/// Trait implemented by backend implementations.
pub trait MonitorBackend {
    /// Refresh the system status data.
    fn update(&mut self, opts: &Options) -> BackendResult<()>;

    /// Get the hostname
    fn hostname(&mut self) -> BackendResult<String>;

    /// Get the kernel
    fn system_version(&mut self) -> BackendResult<String>;

    /// Get the system uptime.
    fn uptime(&mut self) -> BackendResult<Duration>;

    /// Get the number of physical CPU cores.
    fn cpu_count(&mut self) -> BackendResult<u32>;

    /// Get the number of physical CPU cores.
    fn logical_cpu_count(&mut self) -> BackendResult<u32>;

    /// Get overall CPU utilization.
    fn global_cpu(&mut self) -> BackendResult<CPU>;

    /// Get memory usage.
    fn memory(&mut self) -> BackendResult<Memory>;

    /// Get swap usage.
    fn swap(&mut self) -> BackendResult<Swap>;

    /// Get the system load average.
    fn load_avg(&mut self) -> BackendResult<LoadAvg>;

    /// Get the running processes.
    fn processes<'a>(&'a mut self) -> BackendResult<Vec<Process>>;

    /// Get the comamnd information for a process.
    fn process_cmd_info(&mut self, pid: u32) -> BackendResult<ProcessCommandInfo>;

    /// Get the networks.
    fn networks(&mut self) -> BackendResult<Vec<NetworkStats>>;

    /// Get the filesystems.
    fn filesystems(&mut self) -> BackendResult<Vec<Filesystem>>;

    fn has_process_time(&mut self) -> bool;
}
