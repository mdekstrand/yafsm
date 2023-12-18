use log::*;
use procfs::{CpuTime, KernelStats};

use super::data::ProcFSWrapper;
use crate::backend::{util::Diff, BackendError, BackendResult};

/// Total CPU time.
fn total_time(cpu: &CpuTime) -> u64 {
    cpu.user
        + cpu.nice
        + cpu.system
        + cpu.idle
        + cpu.iowait.unwrap_or_default()
        + cpu.irq.unwrap_or_default()
        + cpu.softirq.unwrap_or_default()
        + cpu.steal.unwrap_or_default()
        + cpu.guest.unwrap_or_default()
        + cpu.guest_nice.unwrap_or_default()
}

/// Total used CPU time.
fn total_used(cpu: &CpuTime) -> u64 {
    cpu.user
        + cpu.nice
        + cpu.system
        + cpu.irq.unwrap_or_default()
        + cpu.softirq.unwrap_or_default()
        + cpu.guest.unwrap_or_default()
        + cpu.guest_nice.unwrap_or_default()
}

/// Measure of time spent in different CPU states.
/// Like [CpuTime], but we can construct it and it knows about totals.
#[derive(Debug, Clone)]
pub struct CpuTicks {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: Option<u64>,
    pub irq: Option<u64>,
    pub softirq: Option<u64>,
    pub steal: Option<u64>,
    pub guest: Option<u64>,
    pub guest_nice: Option<u64>,

    pub total: u64,
    pub total_used: u64,
}

impl From<&CpuTime> for CpuTicks {
    fn from(value: &CpuTime) -> Self {
        CpuTicks {
            user: value.user,
            nice: value.nice,
            system: value.system,
            idle: value.idle,
            iowait: value.iowait,
            irq: value.irq,
            softirq: value.softirq,
            steal: value.steal,
            guest: value.guest,
            guest_nice: value.guest_nice,

            total: total_time(value),
            total_used: total_used(value),
        }
    }
}

impl ProcFSWrapper<KernelStats> {
    pub(super) fn cpu_time_diff(&self) -> BackendResult<CpuTicks> {
        let data = self.data()?;
        match (&data.current, &data.previous) {
            (Some(c), Some(p)) => Ok(c.total.diff(&p.total)),
            (Some(c), None) => Ok((&c.total).into()),
            (None, Some(_)) => {
                warn!("update lost data");
                Err(BackendError::NotAvailable)
            }
            (None, None) => {
                warn!("called without update");
                Err(BackendError::NotAvailable)
            }
        }
    }
}

impl Diff for CpuTicks {
    type Difference = CpuTicks;

    fn diff(&self, previous: &Self) -> Self::Difference {
        CpuTicks {
            user: self.user - previous.user,
            nice: self.nice - previous.nice,
            system: self.system - previous.system,
            idle: self.idle - previous.idle,
            iowait: match (self.iowait, previous.iowait) {
                (Some(c), Some(p)) => Some(c - p),
                _ => None,
            },
            irq: match (self.irq, previous.irq) {
                (Some(c), Some(p)) => Some(c - p),
                _ => None,
            },
            softirq: match (self.softirq, previous.softirq) {
                (Some(c), Some(p)) => Some(c - p),
                _ => None,
            },
            steal: match (self.steal, previous.steal) {
                (Some(c), Some(p)) => Some(c - p),
                _ => None,
            },
            guest: match (self.guest, previous.guest) {
                (Some(c), Some(p)) => Some(c - p),
                _ => None,
            },
            guest_nice: match (self.guest_nice, previous.guest_nice) {
                (Some(c), Some(p)) => Some(c - p),
                _ => None,
            },

            total: self.total - previous.total,
            total_used: self.total_used - previous.total_used,
        }
    }
}

impl Diff for CpuTime {
    type Difference = CpuTicks;

    fn diff(&self, previous: &Self) -> Self::Difference {
        let cur: CpuTicks = self.into();
        let prev: CpuTicks = previous.into();
        cur.diff(&prev)
    }
}
