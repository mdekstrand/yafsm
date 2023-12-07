//! Access to the different monitors.
use anyhow::*;
use log::*;
use sysinfo::{CpuRefreshKind, RefreshKind, System, SystemExt};

use self::system::init_system;

pub mod system;

pub struct SystemState {
    pub system: System,
}

impl SystemState {
    pub fn init() -> Result<SystemState> {
        let mut system = init_system()?;
        system.refresh_specifics(RefreshKind::everything());
        Ok(SystemState { system })
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
