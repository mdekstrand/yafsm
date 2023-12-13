//! Process data model.
use std::{cell::RefCell, cmp::Ordering, ops::Deref, time::Duration};

use crate::backend::BackendResult;

use super::{MonitorState, SystemResources};

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessCounts {
    pub running: u32,
    pub sleeping: u32,
    pub other: u32,
}

/// Process list
pub struct ProcessList {
    order: ProcSortOrder,
    procs: Vec<Process>,
    counts: RefCell<Option<ProcessCounts>>,
}

/// Sort order for the process table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcSortOrder {
    CPU,
    Memory,
    IO,
    Time,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub ppid: Option<u32>,
    pub name: String,
    pub uid: Option<u32>,

    pub status: char,
    pub cpu_util: f32,
    pub cpu_time: Option<Duration>,
    pub cpu_utime: Option<Duration>,
    pub cpu_stime: Option<Duration>,

    pub mem_util: f32,
    pub mem_rss: u64,
    pub mem_virt: u64,

    pub io_read: Option<u64>,
    pub io_write: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ProcessCommandInfo {
    pub exe: String,
    pub cmdline: Vec<String>,
}

impl ProcessList {
    pub(super) fn create<'a, 'b>(
        state: &'a MonitorState<'b>,
        procs: Vec<Process>,
    ) -> BackendResult<Self>
    where
        'b: 'a,
    {
        let order = if let Some(order) = state.proc_sort {
            order
        } else if state.global_cpu()?.utilization >= 0.9 {
            ProcSortOrder::CPU
        } else if state.memory()?.used_frac() >= 0.5 {
            ProcSortOrder::Memory
        } else {
            ProcSortOrder::CPU
        };
        Ok(ProcessList {
            order,
            procs,
            counts: RefCell::default(),
        })
    }

    pub fn active_sort_order(&self) -> ProcSortOrder {
        self.order
    }

    /// Sort the list of processes.
    ///
    /// Returns the effective sort order.
    pub fn sort(&mut self) {
        let sort_fn = match self.order {
            ProcSortOrder::CPU => proc_sort_cpu,
            ProcSortOrder::Memory => proc_sort_mem,
            ProcSortOrder::IO => proc_sort_io,
            ProcSortOrder::Time => proc_sort_time,
        };

        self.procs.sort_by(sort_fn);
    }

    pub fn counts(&self) -> ProcessCounts {
        let mut ccache = self.counts.borrow_mut();
        if let Some(counts) = *ccache {
            counts
        } else {
            let mut counts = ProcessCounts::default();
            for proc in &self.procs {
                match proc.status {
                    'R' => counts.running += 1,
                    'S' => counts.sleeping += 1,
                    _ => counts.other += 1,
                }
            }
            *ccache = Some(counts);
            counts
        }
    }
}

fn proc_sort_cpu(p1: &Process, p2: &Process) -> Ordering {
    p2.cpu_util.total_cmp(&p1.cpu_util)
}

fn proc_sort_mem(p1: &Process, p2: &Process) -> Ordering {
    p2.mem_util.total_cmp(&p1.mem_util)
}

fn proc_sort_io(p1: &Process, p2: &Process) -> Ordering {
    if let (Some(r1), Some(r2), Some(w1), Some(w2)) =
        (p1.io_read, p2.io_read, p1.io_write, p2.io_write)
    {
        (r2 + w2).cmp(&(r1 + w1))
    } else {
        Ordering::Equal
    }
}

fn proc_sort_time(p1: &Process, p2: &Process) -> Ordering {
    if let (Some(t1), Some(t2)) = (p1.cpu_time, p2.cpu_time) {
        t2.cmp(&t1)
    } else {
        Ordering::Equal
    }
}

impl Deref for ProcessList {
    type Target = [Process];

    fn deref(&self) -> &Self::Target {
        &self.procs
    }
}
