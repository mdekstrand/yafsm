//! Load and pressure statistics.

/// Load averages.
#[derive(Debug, Clone)]
pub struct LoadAvg {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

/// Full pressure stall information.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SystemPressure {
    pub cpu_psi: Pressure,
    pub mem_psi: Pressure,
    pub mem_full_psi: Pressure,
    pub io_psi: Pressure,
    pub io_full_psi: Pressure,
}

/// Pressure stall record.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Pressure {
    pub avg10: f32,
    pub avg60: f32,
    pub avg300: f32,
    pub total: u64,
}
