//! Filesystem

#[derive(Debug, Clone)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
    pub used: u64,
    pub avail: u64,
    pub total: u64,
}
