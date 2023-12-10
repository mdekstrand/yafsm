//! Process data model.
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Process {
    pub pid: u32,
    pub ppid: Option<u32>,
    pub name: String,
    pub uid: Option<u32>,

    pub status: char,
    pub cpu_util: f32,
    pub cpu_utime: Option<Duration>,
    pub cpu_stime: Option<Duration>,

    pub mem_rss: u64,
    pub mem_virt: u64,

    pub io_read: Option<u64>,
    pub io_write: Option<u64>,
}

pub struct ProcessDetails {
    pub exe: String,
    pub cmdline: Vec<String>,
}
