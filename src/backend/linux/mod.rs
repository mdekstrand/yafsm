//! Linux-specific backend with [procfs].
use std::collections::HashSet;

use etc_os_release::OsRelease;
use gethostname::gethostname;
use log::*;
use nix::sys::statvfs::statvfs;
use procfs::*;
use regex::RegexSet;

mod data;
mod io;
mod kernel;
mod network;

use super::{error::*, util::Tick, MonitorBackend};
use crate::model::*;
use data::ProcFSWrapper;

/// Linux-specific backend.
pub struct LinuxBackend {
    tick: Tick,
    release: BackendResult<OsRelease>,
    cpus: BackendResult<CpuInfo>,
    kernel: ProcFSWrapper<KernelStats>,
    memory: ProcFSWrapper<Meminfo>,

    load: ProcFSWrapper<LoadAverage>,
    cpu_pressure: ProcFSWrapper<CpuPressure>,
    mem_pressure: ProcFSWrapper<MemoryPressure>,
    io_pressure: ProcFSWrapper<IoPressure>,

    net_ifs: ProcFSWrapper<net::InterfaceDeviceStatus>,
    disks: ProcFSWrapper<DiskStats>,
    disk_filters: RegexSet,
    mounts: ProcFSWrapper<Vec<MountEntry>>,
    mount_filters: RegexSet,
}

impl LinuxBackend {
    pub fn create() -> BackendResult<LinuxBackend> {
        let tick = Tick::new();
        Ok(LinuxBackend {
            tick: tick.clone(),
            release: OsRelease::open().map_err(|e| e.into()),
            cpus: CpuInfo::current().map_err(|e| e.into()),
            kernel: ProcFSWrapper::for_curent_si(&tick),
            memory: ProcFSWrapper::for_current(&tick),
            load: ProcFSWrapper::for_current(&tick),
            cpu_pressure: ProcFSWrapper::for_current(&tick),
            mem_pressure: ProcFSWrapper::for_current(&tick),
            io_pressure: ProcFSWrapper::for_current(&tick),
            net_ifs: ProcFSWrapper::for_current(&tick),
            disks: ProcFSWrapper::for_current(&tick),
            disk_filters: RegexSet::new(&[
                r"^loop\d+",
                r"^mmcblk\d+(p|boot)\d+",
                r"^dm-\d+",
                r"^([sh]|xv])d[a-z]+\d+",
            ])
            .unwrap(),
            mounts: ProcFSWrapper::new(mounts, &tick),
            mount_filters: RegexSet::new(&["^/(dev|proc|sys|run)(/|$)"]).unwrap(),
        })
    }
}

impl LinuxBackend {
    fn map_result<T, R, F>(&self, result: &BackendResult<T>, func: F) -> BackendResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        match result {
            Ok(v) => Ok(func(v)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl MonitorBackend for LinuxBackend {
    fn update(&mut self, _opts: &Options) -> BackendResult<()> {
        self.tick.advance();
        trace!("advanced to tick {}", self.tick.current());
        Ok(())
    }

    fn hostname(&self) -> BackendResult<String> {
        Ok(gethostname().to_string_lossy().into())
    }

    fn system_version(&self) -> BackendResult<String> {
        self.map_result(&self.release, |osr| osr.pretty_name().into())
    }

    fn uptime(&self) -> BackendResult<std::time::Duration> {
        let res = Uptime::current()?;
        Ok(res.uptime_duration())
    }

    fn cpu_count(&self) -> BackendResult<u32> {
        // TODO: fix this to get physical cores
        self.map_result(&self.cpus, |cpui| cpui.num_cores() as u32)
    }

    fn logical_cpu_count(&self) -> BackendResult<u32> {
        self.map_result(&self.cpus, |cpui| cpui.num_cores() as u32)
    }

    fn global_cpu(&self) -> BackendResult<CPU> {
        let cpu = self.kernel.cpu_time_diff()?;

        Ok(CPU {
            utilization: cpu.total_used as f32 / cpu.total as f32,
        })
    }

    fn memory(&self) -> BackendResult<Memory> {
        let mem = self.memory.current()?;
        Ok(Memory {
            used: if let Some(avail) = mem.mem_available {
                mem.mem_total - avail
            } else {
                mem.active + mem.inactive
            },
            freeable: if let Some(avail) = mem.mem_available {
                avail - mem.mem_free
            } else {
                mem.cached + mem.buffers
            },
            free: mem.mem_free,
            total: mem.mem_total,
        })
    }

    fn swap(&self) -> BackendResult<Swap> {
        let mem = self.memory.current()?;
        Ok(Swap {
            used: mem.swap_total - mem.swap_free,
            free: mem.swap_free,
            total: mem.swap_total,
        })
    }

    fn load_avg(&self) -> BackendResult<LoadAvg> {
        let load = self.load.current()?;
        Ok(LoadAvg {
            one: load.one,
            five: load.five,
            fifteen: load.fifteen,
        })
    }

    fn pressure(&self) -> BackendResult<SystemPressure> {
        let cp = self.cpu_pressure.current()?;
        let mp = self.mem_pressure.current()?;
        let ip = self.io_pressure.current()?;

        Ok(SystemPressure {
            cpu_psi: Pressure {
                avg10: cp.some.avg10,
                avg60: cp.some.avg60,
                avg300: cp.some.avg300,
                total: cp.some.total,
            },
            mem_psi: Pressure {
                avg10: mp.some.avg10,
                avg60: mp.some.avg60,
                avg300: mp.some.avg300,
                total: mp.some.total,
            },
            mem_full_psi: Pressure {
                avg10: mp.full.avg10,
                avg60: mp.full.avg60,
                avg300: mp.full.avg300,
                total: mp.full.total,
            },
            io_psi: Pressure {
                avg10: ip.some.avg10,
                avg60: ip.some.avg60,
                avg300: ip.some.avg300,
                total: ip.some.total,
            },
            io_full_psi: Pressure {
                avg10: ip.full.avg10,
                avg60: ip.full.avg60,
                avg300: ip.full.avg300,
                total: ip.full.total,
            },
        })
    }

    fn processes<'a>(&'a self) -> BackendResult<Vec<Process>> {
        Err(BackendError::NotSupported)
    }

    fn process_cmd_info(&self, pid: u32) -> BackendResult<ProcessCommandInfo> {
        Err(BackendError::NotSupported)
    }

    fn networks(&self) -> BackendResult<Vec<NetworkStats>> {
        let nets = self.net_ifs.network_usage()?;
        Ok(nets
            .into_iter()
            .map(|n| NetworkStats {
                name: n.name,
                rx_bytes: n.recv_bytes,
                rx_packets: n.recv_packets,
                tx_bytes: n.sent_bytes,
                tx_packets: n.sent_packets,
            })
            .collect())
    }

    fn disks(&self) -> BackendResult<Vec<DiskIO>> {
        let disks = self.disks.disk_stats()?;
        Ok(disks
            .into_iter()
            .filter(|d| !self.disk_filters.is_match(&d.name))
            .collect())
    }

    fn filesystems(&self) -> BackendResult<Vec<Filesystem>> {
        let mounts = self.mounts.current()?;
        let mut res = Vec::with_capacity(mounts.len());
        let mut seen = HashSet::new();
        for me in mounts.iter() {
            let path = me.fs_file.as_str();
            if self.mount_filters.is_match(path) || seen.contains(path) {
                continue;
            }
            seen.insert(path.to_string());
            let stat = statvfs(path)?;
            res.push(Filesystem {
                name: me.fs_spec.clone(),
                mount_point: path.to_string(),
                total: stat.blocks() * stat.block_size(),
                avail: stat.blocks_available() * stat.block_size(),
                used: (stat.blocks() - stat.blocks_available()) * stat.block_size(),
            });
        }
        Ok(res)
    }

    fn has_process_time(&self) -> bool {
        false
    }
}
