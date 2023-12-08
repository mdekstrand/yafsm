//! System monitoring with [sysinfo].

use anyhow::Result;
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

    fn global_cpu(&mut self) -> Result<CPU> {
        Ok(CPU {
            utilization: self.global_cpu_info().cpu_usage(),
        })
    }
    fn memory(&mut self) -> Result<Memory> {
        let used = self.used_memory();
        let total = self.total_memory();
        let free = self.free_memory();
        let freeable = self.available_memory() - free;
        Ok(Memory {
            used,
            freeable,
            free,
            total,
        })
    }

    fn swap(&mut self) -> Result<Swap> {
        Ok(Swap {
            used: self.used_swap(),
            free: self.free_swap(),
            total: self.total_swap(),
        })
    }
}
