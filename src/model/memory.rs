//! Data model for memory information.

/// Basic memory usage statistics.
#[derive(Debug, Clone)]
pub struct Memory {
    /// The memory used (in bytes).
    pub used: u64,
    /// The memory that could be freed for use (in bytes).
    pub freeable: u64,
    /// The free memory (in bytes).
    pub free: u64,
    /// The total memory (in bytes).
    pub total: u64,
    /// Extended memory statistics.
    pub extended: ExtendedMemory,
}

impl Memory {
    /// Compute the used memory as a fraction of total.
    pub fn used_frac(&self) -> f32 {
        self.used as f32 / self.total as f32
    }

    /// Compute the freeable memory as a fraction of total.
    pub fn freeable_frac(&self) -> f32 {
        self.freeable as f32 / self.total as f32
    }
}

#[derive(Debug, Clone)]
pub enum ExtendedMemory {
    None,
    Linux(LinuxMemory),
}

#[derive(Debug, Clone)]
pub struct LinuxMemory {
    pub active: u64,
    pub inactive: u64,
    pub buffers: u64,
    pub cached: u64,
    pub reclaimable: Option<u64>,
    pub arc: Option<u64>,
}
