//! Process data model.
use std::{borrow::Cow, path::Path, time::Duration};

use super::IOUsage;

#[derive(Debug, Clone, Default)]
pub struct Process<'a> {
    pub pid: u32,
    pub ppid: Option<u32>,
    pub name: Cow<'a, str>,
    pub exe: Cow<'a, Path>,
    pub uid: Option<u32>,

    pub status: char,
    pub cpu: f32,
    pub mem_rss: u64,
    pub mem_virt: u64,
    pub io: Option<IOUsage>,

    pub wall_time: Duration,
}
