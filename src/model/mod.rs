//! Access to the different monitors.
use std::time::Duration;

use anyhow::*;
use uzers::{Users, UsersCache};

pub mod cpu;
pub mod disk;
pub mod fs;
pub mod load;
pub mod memory;
pub mod network;
pub mod options;
pub mod process;
pub mod source;
pub mod swap;

pub use cpu::CPU;
pub use disk::DiskIO;
pub use fs::Filesystem;
pub use load::LoadAvg;
pub use memory::Memory;
pub use network::NetworkStats;
pub use options::Options;
pub use process::{ProcSortOrder, Process, ProcessCommandInfo};
pub use swap::Swap;

use crate::backend::{BackendResult, MonitorBackend};

use self::process::ProcessList;
pub use self::source::{NetworkInfo, RunningProcesses, StorageInfo, SystemInfo, SystemResources};

/// Interface for data monitor sources.
///
/// This is defined as a trait so the monitor state can be object-safe, where that might
/// be helpful.  It also has methods that are somewhat duplicative of [MonitorBackend],
/// but many of them handle checking whether that feature should be enabled.
pub trait MonitorData:
    SystemInfo + SystemResources + RunningProcesses + NetworkInfo + StorageInfo
{
    fn backend(&self) -> &dyn MonitorBackend;
    fn lookup_user(&self, uid: u32) -> Result<Option<String>>;
}

/// Container for system monitor state.
pub struct MonitorState<B: MonitorBackend> {
    pub options: Options,
    pub backend: B,
    /// Sort order for processes.  [None] to sort automatically.
    pub proc_sort: Option<ProcSortOrder>,
    pub user_db: UsersCache,
}

impl<B> MonitorState<B>
where
    B: MonitorBackend,
{
    pub fn create(options: Options, backend: B) -> Result<MonitorState<B>> {
        Ok(MonitorState {
            options,
            backend,
            proc_sort: None,
            user_db: UsersCache::new(),
        })
    }

    pub fn refresh(&mut self) -> BackendResult<()> {
        self.backend.update(&self.options)
    }
}

impl<B> MonitorData for MonitorState<B>
where
    B: MonitorBackend,
{
    fn backend(&self) -> &dyn MonitorBackend {
        &self.backend
    }

    fn lookup_user(&self, uid: u32) -> Result<Option<String>> {
        let u = self.user_db.get_user_by_uid(uid);
        Ok(u.map(|u| u.name().to_string_lossy().to_string()))
    }
}

impl<B> SystemInfo for MonitorState<B>
where
    B: MonitorBackend,
{
    fn hostname(&self) -> BackendResult<String> {
        self.backend.hostname()
    }

    fn system_version(&self) -> BackendResult<String> {
        self.backend.system_version()
    }

    fn uptime(&self) -> BackendResult<Duration> {
        self.backend.uptime()
    }
}

impl<B> SystemResources for MonitorState<B>
where
    B: MonitorBackend,
{
    fn cpu_count(&self) -> BackendResult<u32> {
        self.backend.cpu_count()
    }

    fn global_cpu(&self) -> BackendResult<CPU> {
        self.backend.global_cpu()
    }

    fn memory(&self) -> BackendResult<Memory> {
        self.backend.memory()
    }

    fn swap(&self) -> BackendResult<Swap> {
        self.backend.swap()
    }

    fn load_avg(&self) -> BackendResult<LoadAvg> {
        self.backend.load_avg()
    }
}

impl<B> RunningProcesses for MonitorState<B>
where
    B: MonitorBackend,
{
    fn processes(&self) -> BackendResult<ProcessList> {
        ProcessList::create(self, self.backend.processes()?)
    }

    fn process_cmd_info(&self, pid: u32) -> BackendResult<ProcessCommandInfo> {
        self.backend.process_cmd_info(pid)
    }
}

impl<B> NetworkInfo for MonitorState<B>
where
    B: MonitorBackend,
{
    fn networks(&self) -> BackendResult<Vec<NetworkStats>> {
        self.backend.networks()
    }
}

impl<B> StorageInfo for MonitorState<B>
where
    B: MonitorBackend,
{
    fn filesystems(&self) -> BackendResult<Vec<Filesystem>> {
        self.backend.filesystems()
    }
}
