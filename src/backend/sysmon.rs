//! System monitoring with [sysinfo].

use std::time::Duration;

use anyhow::{anyhow, Result};
use log::*;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

use crate::model::*;

use super::MonitorBackend;

pub fn initialize() -> Result<System> {
    let mut sys = System::new();
    sys.refresh_specifics(RefreshKind::everything());
    Ok(sys)
}

impl MonitorBackend for System {
    fn update(&mut self, _opts: &Options) -> Result<()> {
        debug!("refreshing system");
        let specs = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory();
        self.refresh_specifics(specs);
        Ok(())
    }

    fn hostname(&self) -> Result<String> {
        self.host_name().ok_or(anyhow!("no host name"))
    }

    fn system_version(&self) -> Result<String> {
        let os = self.distribution_id();
        let osv = self.os_version().ok_or(anyhow!("no system version"))?;
        let k = self.name().ok_or(anyhow!("no system name"))?;
        let kv = self.kernel_version().ok_or(anyhow!("no kernel version"))?;
        Ok(format!("{} {} with {} {}", os, osv, k, kv))
    }

    fn uptime(&self) -> Result<Duration> {
        Ok(Duration::from_secs(SystemExt::uptime(self)))
    }

    fn cpu_count(&self) -> Result<u32> {
        Ok(self.cpus().len() as u32)
    }

    fn global_cpu(&self) -> Result<CPU> {
        Ok(CPU {
            utilization: self.global_cpu_info().cpu_usage() / 100.0,
        })
    }

    fn memory(&self) -> Result<Memory> {
        let used = self.used_memory();
        let total = self.total_memory();
        let free = self.free_memory();
        let freeable = self.available_memory().saturating_sub(free);
        Ok(Memory {
            used,
            freeable,
            free,
            total,
        })
    }

    fn swap(&self) -> Result<Swap> {
        Ok(Swap {
            used: self.used_swap(),
            free: self.free_swap(),
            total: self.total_swap(),
        })
    }

    fn load_avg(&self) -> Result<LoadAvg> {
        let la = self.load_average();
        Ok(LoadAvg {
            one: la.one as f32,
            five: la.five as f32,
            fifteen: la.fifteen as f32,
        })
    }
}
