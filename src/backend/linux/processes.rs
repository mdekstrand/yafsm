//! Linux process code.
use std::{collections::HashMap, time::Instant};

use log::*;
use procfs::process::{all_processes, Io, Stat};
use procfs::{Meminfo, ProcResult, WithCurrentSystemInfo};

use crate::backend::linux::kernel::ticks_to_duration;
use crate::backend::util::window_norm_u64;
use crate::backend::{BackendResult, MonitorBackend};
use crate::model::Process;

use super::kernel::CpuTicks;
use super::LinuxBackend;

pub(super) struct ProcessRecord {
    pub pid: i32,
    pub uid: Option<u32>,
    pub stat: Stat,
    pub io: Option<Io>,
    pub fetched: Instant,
}

impl ProcessRecord {
    pub(super) fn load_all() -> ProcResult<HashMap<i32, ProcessRecord>> {
        let mut procs = HashMap::new();
        for proc in all_processes()? {
            let proc = match proc {
                Ok(p) => p,
                Err(e) => {
                    warn!("error fetching process: {}", e);
                    continue;
                }
            };
            let stat = match proc.stat() {
                Ok(s) => s,
                Err(e) => {
                    warn!("process {}: error fetching stat: {}", proc.pid, e);
                    continue;
                }
            };
            let io = proc.io().ok();
            procs.insert(
                proc.pid,
                ProcessRecord {
                    pid: proc.pid,
                    uid: proc.uid().ok(),
                    stat,
                    io,
                    fetched: Instant::now(),
                },
            );
        }
        Ok(procs)
    }
}

impl LinuxBackend {
    pub(super) fn process_info(
        &self,
        cur: &ProcessRecord,
        prev: Option<&ProcessRecord>,
        cpu: &CpuTicks,
        mem: &Meminfo,
    ) -> BackendResult<Process> {
        trace!("looking up process {}", cur.pid);
        let time = cur.stat.utime + cur.stat.stime;
        let ncpus = self.cpu_count()?;
        let rss = cur.stat.rss_bytes().get();
        let mut proc = Process {
            pid: cur.pid as u32,
            ppid: Some(cur.stat.ppid as u32),
            name: cur.stat.comm.clone(),
            uid: cur.uid,
            status: cur.stat.state,
            cpu_util: 0.0,
            cpu_time: Some(ticks_to_duration(time)),
            cpu_utime: Some(ticks_to_duration(cur.stat.utime)),
            cpu_stime: Some(ticks_to_duration(cur.stat.stime)),
            mem_util: rss as f32 / mem.mem_total as f32,
            mem_rss: rss,
            mem_virt: cur.stat.vsize,
            io_read: None,
            io_write: None,
        };
        if let Some(io) = cur.io {
            proc.io_read = Some(io.read_bytes);
            proc.io_write = Some(io.write_bytes);
        }
        if let Some(prev) = prev {
            let delta_t = cur.fetched.duration_since(prev.fetched);
            let pt = prev.stat.utime + prev.stat.stime;
            proc.cpu_time = proc.cpu_time.map(|t| t - ticks_to_duration(pt));
            proc.cpu_utime = proc
                .cpu_utime
                .map(|t| t - ticks_to_duration(prev.stat.utime));
            proc.cpu_stime = proc
                .cpu_stime
                .map(|t| t - ticks_to_duration(prev.stat.stime));
            if let Some(io) = prev.io {
                proc.io_read = proc
                    .io_read
                    .map(|b| window_norm_u64(b - io.read_bytes, delta_t));
                proc.io_write = proc
                    .io_write
                    .map(|b| window_norm_u64(b - io.write_bytes, delta_t));
            }
            let tdiff = time - pt;
            proc.cpu_util = (tdiff * ncpus as u64) as f32 / cpu.total as f32;
        }
        Ok(proc)
    }
}
