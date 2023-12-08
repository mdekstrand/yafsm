//! CPU usage data model.

/// Basic CPU usage data.
#[derive(Debug, Clone)]
pub struct CPU {
    /// The current utilization (as a fraction).
    pub utilization: f32,
}
