//! Filesystem

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
    pub used: u64,
    pub avail: u64,
    pub total: u64,
}

impl Filesystem {
    pub fn utilization(&self) -> f32 {
        // compute in 64-bit to reduce risk of range errors
        (self.used as f64 / self.total as f64) as f32
    }
}
