//! System monitoring with [sysinfo].

use anyhow::Result;
use sysinfo::{System, SystemExt};

pub fn init_system() -> Result<System> {
    let mut sys = System::new();
    sys.refresh_all();
    Ok(sys)
}
