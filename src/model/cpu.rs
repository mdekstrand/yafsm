//! CPU usage data model.

/// Basic CPU usage data.
#[derive(Debug, Clone)]
pub struct CPU {
    /// The current utilization (as a fraction).
    pub utilization: f32,

    /// Extended CPU info.
    pub extended: CPUExt,
}

#[derive(Debug, Clone)]
pub enum CPUExt {
    None,
    Linux(LinuxCPU),
}

/// Linux-specific CPU statistics.
#[derive(Debug, Clone)]
pub struct LinuxCPU {
    pub user: f32,
    pub system: f32,
    pub iowait: f32,
    pub idle: f32,
    pub irq: f32,
    pub nice: f32,
    pub steal: f32,
}
