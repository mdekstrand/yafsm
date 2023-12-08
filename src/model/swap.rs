//! Swap data structures
#[derive(Debug, Clone)]
pub struct Swap {
    /// The swap used (in bytes).
    pub used: u64,
    /// The free swap (in bytes).
    pub free: u64,
    /// The total swap (in bytes).
    pub total: u64,
}

impl Swap {
    /// Compute the used memory as a fraction of total.
    pub fn used_frac(&self) -> f32 {
        self.used as f32 / self.total as f32
    }
}
