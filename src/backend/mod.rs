//! Backend data collector implementations.
use std::time::Duration;

use anyhow::Result;

use crate::model::*;

pub mod sysmon;

/// Trait implemented by backend implementations.
pub trait MonitorBackend {
    /// Refresh the system status data.
    fn update(&mut self, opts: &Options) -> Result<()>;

    /// Get the hostname
    fn hostname(&self) -> Result<String>;

    /// Get the kernel
    fn system_version(&self) -> Result<String>;

    /// Get the system uptime.
    fn uptime(&self) -> Result<Duration>;

    /// Get the number of physical CPU cores.
    fn cpu_count(&self) -> Result<u32>;

    /// Get the number of physical CPU cores.
    fn logical_cpu_count(&self) -> Result<u32>;

    /// Get overall CPU utilization.
    fn global_cpu(&self) -> Result<CPU>;

    /// Get memory usage.
    fn memory(&self) -> Result<Memory>;

    /// Get swap usage.
    fn swap(&self) -> Result<Swap>;

    /// Get the system load average.
    fn load_avg(&self) -> Result<LoadAvg>;

    /// Get the running processes.
    fn processes<'a>(&'a self) -> Result<Vec<Process>>;

    /// Get the comamnd information for a process.
    fn process_details(&self, pid: u32) -> Result<ProcessDetails>;

    fn has_process_time(&self) -> bool;
}
