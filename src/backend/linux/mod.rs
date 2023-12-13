//! Linux-specific backend with [procfs].
use etc_os_release::OsRelease;
use gethostname::gethostname;
use procfs::*;

mod kernel;

use super::{error::*, util::Tick, MonitorBackend};

/// Linux-specific backend.
pub struct LinuxBackend {
    tick: Tick,
    release: BackendResult<OsRelease>,
    cpus: BackendResult<CpuInfo>,
    kernel: kernel::Stats,
}

impl LinuxBackend {
    pub fn create() -> BackendResult<LinuxBackend> {
        let tick = Tick::new();
        Ok(LinuxBackend {
            tick: tick.clone(),
            release: OsRelease::open().map_err(|e| e.into()),
            cpus: CpuInfo::current().map_err(|e| e.into()),
            kernel: kernel::Stats::new(tick.clone()),
        })
    }
}

impl LinuxBackend {
    fn map_result<T, R, F>(&self, result: &BackendResult<T>, func: F) -> BackendResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        match result {
            Ok(v) => Ok(func(v)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl MonitorBackend for LinuxBackend {
    fn update(&mut self, _opts: &crate::model::Options) -> BackendResult<()> {
        self.tick.advance();
        Ok(())
    }

    fn hostname(&self) -> BackendResult<String> {
        Ok(gethostname().to_string_lossy().into())
    }

    fn system_version(&self) -> BackendResult<String> {
        self.map_result(&self.release, |osr| osr.pretty_name().into())
    }

    fn uptime(&self) -> BackendResult<std::time::Duration> {
        let res = Uptime::current()?;
        Ok(res.uptime_duration())
    }

    fn cpu_count(&self) -> BackendResult<u32> {
        // TODO: fix this to get physical cores
        self.map_result(&self.cpus, |cpui| cpui.num_cores() as u32)
    }

    fn logical_cpu_count(&self) -> BackendResult<u32> {
        self.map_result(&self.cpus, |cpui| cpui.num_cores() as u32)
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
