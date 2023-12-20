//! GPU statistics

#[derive(Debug, Clone)]
pub struct GPUInfo {
    pub name: String,
    pub cpu: f32,
    pub mem: f32,
    pub temp: u32,
}
