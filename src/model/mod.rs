//! Access to the different monitors.
use std::time::Duration;

use anyhow::*;
use uzers::{Users, UsersCache};

pub mod cpu;
pub mod disk;
pub mod fs;
pub mod gpu;
pub mod load;
pub mod memory;
pub mod network;
pub mod options;
pub mod process;
pub mod source;
pub mod state;
pub mod swap;

pub use cpu::CPU;
pub use disk::DiskIO;
pub use fs::Filesystem;
pub use gpu::GPUStats;
pub use load::{LoadAvg, SystemPressure};
pub use memory::{ExtendedMemory, Memory};
pub use network::NetworkStats;
pub use options::Options;
pub use process::{ProcSortOrder, Process, ProcessCommandInfo};
pub use source::{
    GPUInfo, NetworkInfo, RunningProcesses, StorageInfo, SystemInfo, SystemResources,
};
pub use state::MonitorState;
pub use swap::Swap;

use crate::backend::MonitorBackend;

/// Interface for data monitor sources.
///
/// This is defined as a trait so the monitor state can be object-safe, where that might
/// be helpful.  It also has methods that are somewhat duplicative of [MonitorBackend],
/// but many of them handle checking whether that feature should be enabled.
pub trait MonitorData:
    SystemInfo + SystemResources + RunningProcesses + NetworkInfo + StorageInfo + GPUInfo
{
    fn backend(&self) -> &dyn MonitorBackend;
    fn lookup_user(&self, uid: u32) -> Result<Option<String>>;
}
