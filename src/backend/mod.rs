//! Backend data collector implementations.
use anyhow::Result;

use crate::model::{cpu::CPU, memory::Memory, swap::Swap, Options};

pub mod sysmon;

/// Trait implemented by backend implementations.
pub trait MonitorBackend {
    /// Refresh the system status data.
    fn update(&mut self, opts: &Options) -> Result<()>;

    /// Get CPU utilization.
    fn global_cpu(&mut self) -> Result<CPU>;

    /// Get memory usage.
    fn memory(&mut self) -> Result<Memory>;

    /// Get swap usage.
    fn swap(&mut self) -> Result<Swap>;
}
