//! Access to the different monitors.
use anyhow::*;
use log::*;
use sysinfo::{System, SystemExt};

use self::system::init_system;

pub mod system;

pub struct SystemState {
    pub system: System,
}

impl SystemState {
    pub fn init() -> Result<SystemState> {
        let system = init_system()?;
        Ok(SystemState { system })
    }

    pub fn refresh(&mut self) -> Result<()> {
        debug!("refreshing system");
        self.system.refresh_all();
        Ok(())
    }
}
