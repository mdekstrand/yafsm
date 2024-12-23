//! Monitor state.
use crate::backend::{BackendResult, MonitorBackend};

use super::{process::ProcessList, *};

/// Container for system monitor state.
pub struct MonitorState<'back> {
    pub options: Options,
    /// Sort order for processes.  [None] to sort automatically.
    pub proc_sort: Option<ProcSortOrder>,

    pub backend: &'back mut dyn MonitorBackend,
    pub user_db: UsersCache,
}

impl<'back> MonitorState<'back> {
    pub fn create(
        options: Options,
        backend: &'back mut dyn MonitorBackend,
    ) -> Result<MonitorState<'back>> {
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

impl<'back> MonitorData for MonitorState<'back> {
    fn backend(&self) -> &dyn MonitorBackend {
        self.backend
    }

    fn lookup_user(&self, uid: u32) -> Result<Option<String>> {
        let u = self.user_db.get_user_by_uid(uid);
        Ok(u.map(|u| u.name().to_string_lossy().to_string()))
    }
}

impl<'back> SystemInfo for MonitorState<'back> {
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

impl<'back> SystemResources for MonitorState<'back> {
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

    fn pressure(&self) -> BackendResult<SystemPressure> {
        self.backend.pressure()
    }
}

impl<'back> RunningProcesses for MonitorState<'back> {
    fn processes(&self) -> BackendResult<ProcessList> {
        ProcessList::create(self, self.backend.processes()?)
    }

    fn process_cmd_info(&self, pid: u32) -> BackendResult<ProcessCommandInfo> {
        self.backend.process_cmd_info(pid)
    }
}

impl<'back> NetworkInfo for MonitorState<'back> {
    fn networks(&self) -> BackendResult<Vec<NetworkStats>> {
        self.backend.networks()
    }
}

impl<'back> StorageInfo for MonitorState<'back> {
    fn disk_io(&self) -> BackendResult<Vec<DiskIO>> {
        self.backend.disks()
    }

    fn filesystems(&self) -> BackendResult<Vec<Filesystem>> {
        self.backend.filesystems()
    }
}

impl<'back> GPUInfo for MonitorState<'back> {
    fn gpus(&self) -> BackendResult<Vec<GPUStats>> {
        self.backend.gpus()
    }
}
