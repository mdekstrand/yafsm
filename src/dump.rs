//! Code to dump system information (mostly for testing and debug).
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::{Args, ValueEnum};
use friendly::bytes;
use log::*;

use crate::backend::MonitorBackend;
use crate::model::source::SystemInfo;
use crate::model::*;

#[derive(ValueEnum, Clone, Debug)]
enum DumpType {
    Cpu,
    Mem,
    Procs,
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
            "system info for {} ({})",
            state.hostname()?,
            state.system_version()?,
        );

        for dump in &self.dumps {
            match dump {
                DumpType::Cpu => self.dump_cpu(state)?,
                DumpType::Mem => self.dump_memory(state)?,
                DumpType::Procs => self.dump_processes(state)?,
            }
        }

        Ok(())
    }

    fn dump_cpu(&self, state: &dyn MonitorData) -> Result<()> {
        let cpu = state.global_cpu()?;
        println!("CPU: {:5.1}%", cpu.utilization);

        Ok(())
    }

    fn dump_memory(&self, state: &dyn MonitorData) -> Result<()> {
        let mem = state.memory()?;
        let swap = state.swap()?;

        println!("MEM: {} / {} used", bytes(mem.used), bytes(mem.total));
        println!("SWP: {} / {} used", bytes(swap.used), bytes(swap.total));

        Ok(())
    }

    fn dump_processes(&self, state: &dyn MonitorData) -> Result<()> {
        let procs = state.processes()?;
        info!("dumping {} processes", procs.len());
        for proc in procs.iter() {
            println!("{:?}", proc);
        }
        Ok(())
    }
}
