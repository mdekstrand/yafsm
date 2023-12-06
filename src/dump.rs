//! Code to dump system information (mostly for testing and debug).
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::Args;
use log::*;
use sysinfo::{CpuExt, SystemExt};

use crate::monitors::SystemState;

#[derive(Args, Debug)]
pub struct DumpOpts {
    /// Period of time (in ms) to wait to refresh for system usage information.
    #[arg(long = "dump-wait", default_value = "500")]
    dump_wait: u64,
    /// Dump CPU and usage information.
    #[arg(long = "dump-cpu")]
    dump_cpu: bool,
}

struct DumpState<'a> {
    opts: &'a DumpOpts,
    state: &'a mut SystemState,
    slept: bool,
    dumped: bool,
}

impl DumpOpts {
    pub fn active(&self) -> bool {
        self.dump_cpu
    }

    pub fn dump(&self, state: &mut SystemState) -> Result<bool> {
        let mut ds = DumpState {
            opts: self,
            state,
            slept: false,
            dumped: false,
        };
        ds.dump()
    }
}

impl<'a> DumpState<'a> {
    fn dump(&mut self) -> Result<bool> {
        if self.opts.dump_cpu {
            self.dump_cpu()?;
            self.dumped = true;
        }
        Ok(self.dumped)
    }

    fn ensure_update(&mut self) -> Result<()> {
        if !self.slept {
            let wait = Duration::from_millis(self.opts.dump_wait);
            debug!("waiting {} to refresh", friendly::duration(wait));
            sleep(wait);
            self.state.refresh()?;
            self.slept = true;
        }
        Ok(())
    }

    fn dump_cpu(&mut self) -> Result<()> {
        self.ensure_update()?;

        let cpus = self.state.system.cpus();
        for cpu in cpus {
            println!(
                "CPU {}: {:5.1}% @ {} hz",
                cpu.name(),
                cpu.cpu_usage(),
                cpu.frequency()
            );
        }

        Ok(())
    }
}
