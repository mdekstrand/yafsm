//! Access to the different monitors.
use anyhow::*;
use log::*;
use sysinfo::{CpuRefreshKind, RefreshKind, System, SystemExt};

use crate::backend::sysmon::init_system;

pub mod options;

pub use options::Options;

pub struct SystemMonitor {
    pub options: Options,
    pub system: System,
}

impl SystemMonitor {
    pub fn init(options: Options) -> Result<SystemMonitor> {
        let mut system = init_system()?;
        system.refresh_specifics(RefreshKind::everything());
        Ok(SystemMonitor { options, system })
    }

    pub fn refresh(&mut self) -> Result<()> {
        debug!("refreshing system");
        let specs = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory();
        self.system.refresh_specifics(specs);
        Ok(())
    }
}
