//! Code to dump system information (mostly for testing and debug).
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::{Args, ValueEnum};
use friendly::{bytes, scalar};
use log::*;
use sysinfo::{CpuExt, SystemExt};

use crate::backend::MonitorBackend;
use crate::model::*;

#[derive(ValueEnum, Clone, Debug)]
enum DumpType {
    Cpu,
    Mem,
}

#[derive(Args, Debug)]
pub struct DumpOpts {
    /// Period of time (in ms) to wait to refresh for system usage information.
    #[arg(long = "dump-wait", default_value = "500")]
    dump_wait: u64,
    /// Dump a system status.
    #[arg(short = 'D', long = "dump", id = "ASPECT")]
    dumps: Vec<DumpType>,
}

impl DumpOpts {
    pub fn requested(&self) -> bool {
        !self.dumps.is_empty()
    }

    pub fn dump<B>(&self, state: &mut MonitorState<B>) -> Result<()>
    where
        B: MonitorBackend,
    {
        let wait = Duration::from_millis(self.dump_wait);
        debug!("waiting {} to refresh", friendly::duration(wait));
        sleep(wait);
        state.refresh()?;
        info!(
            "database info for {} ({} {})",
            state.system.host_name().unwrap_or("<unnamed>".into()),
            state.system.distribution_id(),
            state.system.os_version().unwrap_or_default(),
        );

        for dump in &self.dumps {
            match dump {
                DumpType::Cpu => self.dump_cpu(&state.backend)?,
                DumpType::Mem => self.dump_memory(&state.backend)?,
            }
        }

        Ok(())
    }

    fn dump_cpu(&self, state: &dyn MonitorBackend) -> Result<()> {
        let cpus = state.system.cpus();
        for cpu in cpus {
            println!(
                "CPU {}: {:5.1}% @ {}",
                cpu.name(),
                cpu.cpu_usage(),
                scalar(cpu.frequency() * 1000_000).suffix("Hz")
            );
        }

        Ok(())
    }

    fn dump_memory(&self, state: &dyn MonitorBackend) -> Result<()> {
        let mem = state.memory()?;
        let swap = state.swap()?;

        println!("MEM: {} / {} used", bytes(mem.used), bytes(mem.total));
        println!("SWP: {} / {} used", bytes(swap.used), bytes(swap.total));

        Ok(())
    }
}
