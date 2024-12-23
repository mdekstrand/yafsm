//! GPU statistics

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GPUStats {
    pub name: String,
    pub gpu_util: f32,
    pub mem_util: f32,
    pub mem_total: u64,
    pub mem_avail: u64,
    pub mem_used: u64,
    pub temp: Option<f32>,
    /// Power usage in watts
    pub power: Option<f32>,
}
