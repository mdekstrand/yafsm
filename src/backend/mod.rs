//! Backend data collector implementations.
use std::time::Duration;

use anyhow::Result;

use crate::model::{cpu::CPU, memory::Memory, swap::Swap, Options};

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

    /// Get CPU utilization.
    fn global_cpu(&self) -> Result<CPU>;

    /// Get memory usage.
    fn memory(&self) -> Result<Memory>;

    /// Get swap usage.
    fn swap(&self) -> Result<Swap>;
}
