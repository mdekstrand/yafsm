//! System monitoring with [sysinfo].

use std::time::Duration;

use itertools::Itertools;
use log::*;
use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, Networks, Pid, ProcessRefreshKind, RefreshKind,
    System,
};

use crate::model::*;

use super::{error::generic_err, util::RefreshRecord, BackendResult, MonitorBackend};

/// Backend using [sysinfo].
pub struct SysInfoBackend {
    system: System,
    disks: Disks,
    networks: Networks,
    clock: RefreshRecord,
}

impl SysInfoBackend {
    /// Create a new backend.
    pub fn create() -> BackendResult<SysInfoBackend> {
        let mut system = System::new();
        let mut disks = Disks::new();
        let mut networks = Networks::new();
        system.refresh_specifics(RefreshKind::everything());
        disks.refresh(true);
        networks.refresh(true);
        Ok(SysInfoBackend {
            system,
            disks,
            networks,
            clock: RefreshRecord::new(),
        })
    }
}

impl MonitorBackend for SysInfoBackend {
    fn update(&mut self, _opts: &Options) -> BackendResult<()> {
        debug!("refreshing system");
        let specs = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything());
        self.system.refresh_specifics(specs);
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.clock.update();
        Ok(())
    }

    fn hostname(&self) -> BackendResult<String> {
        System::host_name().ok_or(generic_err("no host name"))
    }

    fn system_version(&self) -> BackendResult<String> {
        let os = System::distribution_id();
        let osv = System::os_version().ok_or(generic_err("no system version"))?;
        let k = System::name().ok_or(generic_err("no system name"))?;
        let kv = System::kernel_version().ok_or(generic_err("no kernel version"))?;
        Ok(format!("{} {} with {} {}", os, osv, k, kv))
    }

    fn uptime(&self) -> BackendResult<Duration> {
        Ok(Duration::from_secs(System::uptime()))
    }

    fn cpu_count(&self) -> BackendResult<u32> {
        System::physical_core_count()
            .map(|s| s as u32)
            .ok_or(generic_err("CPU count unavailable"))
    }

    fn logical_cpu_count(&self) -> BackendResult<u32> {
        Ok(self.system.cpus().len() as u32)
    }

    fn global_cpu(&self) -> BackendResult<CPU> {
        Ok(CPU {
            utilization: self.system.global_cpu_usage() / 100.0,
            extended: cpu::ExtendedCPU::None,
        })
    }

    fn memory(&self) -> BackendResult<Memory> {
        let used = self.system.used_memory();
        let total = self.system.total_memory();
        let free = self.system.free_memory();
        let freeable = self.system.available_memory().saturating_sub(free);
        Ok(Memory {
            used,
            freeable,
            free,
            total,
            extended: ExtendedMemory::None,
        })
    }

    fn swap(&self) -> BackendResult<Swap> {
        Ok(Swap {
            used: self.system.used_swap(),
            free: self.system.free_swap(),
            total: self.system.total_swap(),
        })
    }

    fn load_avg(&self) -> BackendResult<LoadAvg> {
        let la = System::load_average();
        Ok(LoadAvg {
            one: la.one as f32,
            five: la.five as f32,
            fifteen: la.fifteen as f32,
        })
    }

    fn processes<'a>(&'a self) -> BackendResult<Vec<Process>> {
        let procs = self.system.processes();
        let mut out = Vec::with_capacity(procs.len());
        for proc in procs.values() {
            let disk = proc.disk_usage();
            out.push(Process {
                pid: proc.pid().as_u32(),
                ppid: proc.parent().map(|p| p.as_u32()),
                name: proc.name().to_string_lossy().to_string(),
                uid: proc.user_id().map(|u| **u),
                status: match proc.status() {
                    sysinfo::ProcessStatus::Idle => 'I',
                    sysinfo::ProcessStatus::Run => 'R',
                    sysinfo::ProcessStatus::Sleep => 'S',
                    sysinfo::ProcessStatus::Stop => 'T',
                    sysinfo::ProcessStatus::Zombie => 'Z',
                    sysinfo::ProcessStatus::Tracing => 't',
                    sysinfo::ProcessStatus::Dead => 'X',
                    sysinfo::ProcessStatus::Wakekill => 'K',
                    sysinfo::ProcessStatus::Waking => 'W',
                    sysinfo::ProcessStatus::Parked => 'P',
                    sysinfo::ProcessStatus::LockBlocked => 'L',
                    sysinfo::ProcessStatus::UninterruptibleDiskSleep => 'D',
                    sysinfo::ProcessStatus::Unknown(_) => '?',
                },
                cpu_util: proc.cpu_usage() / 100.0,
                cpu_time: None,
                cpu_utime: None,
                cpu_stime: None,
                mem_util: proc.memory() as f32 / self.memory()?.total as f32,
                mem_rss: proc.memory(),
                mem_virt: proc.virtual_memory(),
                io_read: Some(self.clock.norm_u64(disk.read_bytes)),
                io_write: Some(self.clock.norm_u64(disk.written_bytes)),
            })
        }
        Ok(out)
    }

    fn process_cmd_info(&self, pid: u32) -> BackendResult<ProcessCommandInfo> {
        let procs = self.system.processes();
        let pid = Pid::from_u32(pid);
        let proc = procs.get(&pid).ok_or(generic_err("missing process"))?;
        Ok(ProcessCommandInfo {
            exe: proc
                .exe()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".into()),
            cmdline: proc
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
        })
    }

    fn networks(&self) -> BackendResult<Vec<NetworkStats>> {
        Ok(self
            .networks
            .into_iter()
            .map(|(name, stats)| NetworkStats {
                name: name.clone(),
                rx_bytes: self.clock.norm_u64(stats.received()),
                tx_bytes: self.clock.norm_u64(stats.transmitted()),
                rx_packets: self.clock.norm_u64(stats.packets_received()),
                tx_packets: self.clock.norm_u64(stats.packets_transmitted()),
            })
            .collect())
    }

    fn filesystems(&self) -> BackendResult<Vec<Filesystem>> {
        Ok(self
            .disks
            .into_iter()
            .map(|d| Filesystem {
                name: d.name().to_string_lossy().into(),
                mount_point: format!("{}", d.mount_point().display()),
                total: d.total_space(),
                avail: d.available_space(),
                used: d.total_space() - d.available_space(),
            })
            .collect_vec())
    }

    fn has_process_time(&self) -> bool {
        false
    }
}
