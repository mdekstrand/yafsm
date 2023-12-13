//! Linux-specific backend with [procfs].
use gethostname::gethostname;
use procfs::*;

use super::{error::*, MonitorBackend};

/// Linux-specific backend.
pub struct LinuxBackend {
    tick: u64,
}

impl LinuxBackend {
    pub fn create() -> BackendResult<LinuxBackend> {
        Ok(LinuxBackend { tick: 0 })
    }
}

impl MonitorBackend for LinuxBackend {
    fn update(&mut self, _opts: &crate::model::Options) -> BackendResult<()> {
        self.tick += 1;
        Ok(())
    }

    fn hostname(&self) -> BackendResult<String> {
        Ok(gethostname().to_string_lossy().into())
    }

    fn system_version(&self) -> BackendResult<String> {
        Err(BackendError::NotSupported)
    }

    fn uptime(&self) -> BackendResult<std::time::Duration> {
        let res = Uptime::current()?;
        Ok(res.uptime_duration())
    }

    fn cpu_count(&self) -> BackendResult<u32> {
        Err(BackendError::NotSupported)
    }

    fn logical_cpu_count(&self) -> BackendResult<u32> {
        Err(BackendError::NotSupported)
    }

    fn global_cpu(&self) -> BackendResult<crate::model::CPU> {
        Err(BackendError::NotSupported)
    }

    fn memory(&self) -> BackendResult<crate::model::Memory> {
        Err(BackendError::NotSupported)
    }

    fn swap(&self) -> BackendResult<crate::model::Swap> {
        Err(BackendError::NotSupported)
    }

    fn load_avg(&self) -> BackendResult<crate::model::LoadAvg> {
        Err(BackendError::NotSupported)
    }

    fn processes<'a>(&'a self) -> BackendResult<Vec<crate::model::Process>> {
        Err(BackendError::NotSupported)
    }

    fn process_cmd_info(&self, pid: u32) -> BackendResult<crate::model::ProcessCommandInfo> {
        Err(BackendError::NotSupported)
    }

    fn networks(&self) -> BackendResult<Vec<crate::model::NetworkStats>> {
        Err(BackendError::NotSupported)
    }

    fn filesystems(&self) -> BackendResult<Vec<crate::model::Filesystem>> {
        Err(BackendError::NotSupported)
    }

    fn has_process_time(&self) -> bool {
        false
    }
}
