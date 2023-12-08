//! Access to the different monitors.
use anyhow::*;
use log::*;
use sysinfo::{CpuRefreshKind, RefreshKind, System, SystemExt};

use crate::backend::sysmon::init_system;

pub mod options;

pub use options::Options;

pub struct SystemStatus {
    pub options: Options,
    pub system: System,
}

impl SystemStatus {
    pub fn init(options: Options) -> Result<SystemStatus> {
        let mut system = init_system()?;
        system.refresh_specifics(RefreshKind::everything());
        Ok(SystemStatus { options, system })
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
