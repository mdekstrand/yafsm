//! Linux-specific backend with [procfs].
use etc_os_release::OsRelease;
use gethostname::gethostname;
use procfs::*;

mod data;
mod kernel;

use super::{error::*, util::Tick, MonitorBackend};
use crate::model::*;
use data::ProcFSData;

/// Linux-specific backend.
pub struct LinuxBackend {
    tick: Tick,
    release: BackendResult<OsRelease>,
    cpus: BackendResult<CpuInfo>,
    kernel: ProcFSData<KernelStats>,
}

impl LinuxBackend {
    pub fn create() -> BackendResult<LinuxBackend> {
        let tick = Tick::new();
        Ok(LinuxBackend {
            tick: tick.clone(),
            release: OsRelease::open().map_err(|e| e.into()),
            cpus: CpuInfo::current().map_err(|e| e.into()),
            kernel: ProcFSData::for_curent_si(&tick),
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
    fn update(&mut self, _opts: &Options) -> BackendResult<()> {
        self.tick.advance();
        Ok(())
    }

    fn hostname(&mut self) -> BackendResult<String> {
        Ok(gethostname().to_string_lossy().into())
    }

    fn system_version(&mut self) -> BackendResult<String> {
        self.map_result(&self.release, |osr| osr.pretty_name().into())
    }

    fn uptime(&mut self) -> BackendResult<std::time::Duration> {
        let res = Uptime::current()?;
        Ok(res.uptime_duration())
    }

    fn cpu_count(&mut self) -> BackendResult<u32> {
        // TODO: fix this to get physical cores
        self.map_result(&self.cpus, |cpui| cpui.num_cores() as u32)
    }

    fn logical_cpu_count(&mut self) -> BackendResult<u32> {
        self.map_result(&self.cpus, |cpui| cpui.num_cores() as u32)
    }

    fn global_cpu(&mut self) -> BackendResult<CPU> {
        // self.kernel.update_if_needed();
        let cpu = self
            .kernel
            .cpu_time_diff()
            .ok_or(BackendError::NotAvailable)?;

        Ok(CPU {
            utilization: cpu.total as f32 / cpu.total_used as f32,
        })
    }

    fn memory(&mut self) -> BackendResult<Memory> {
        Err(BackendError::NotSupported)
    }

    fn swap(&mut self) -> BackendResult<Swap> {
        Err(BackendError::NotSupported)
    }

    fn load_avg(&mut self) -> BackendResult<LoadAvg> {
        Err(BackendError::NotSupported)
    }

    fn processes<'a>(&'a mut self) -> BackendResult<Vec<Process>> {
        Err(BackendError::NotSupported)
    }

    fn process_cmd_info(&mut self, pid: u32) -> BackendResult<ProcessCommandInfo> {
        Err(BackendError::NotSupported)
    }

    fn networks(&mut self) -> BackendResult<Vec<NetworkStats>> {
        Err(BackendError::NotSupported)
    }

    fn filesystems(&mut self) -> BackendResult<Vec<Filesystem>> {
        Err(BackendError::NotSupported)
    }

    fn has_process_time(&mut self) -> bool {
        false
    }
}
