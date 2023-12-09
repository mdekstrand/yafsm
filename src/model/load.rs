//! Load and pressure statistics.

/// Load averages.
#[derive(Debug, Clone)]
pub struct LoadAvg {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}
