use super::*;

pub trait SystemInfo {
    /// Get the hostname.
    fn hostname(&self) -> Result<String>;

    /// Get the system version information.
    fn system_version(&self) -> Result<String>;

    /// Get the system uptime.
    fn uptime(&self) -> Result<Duration>;
}

pub trait SystemResources {
    /// Get CPU utilization.
    fn global_cpu(&self) -> Result<CPU>;

    /// Get memory usage.
    fn memory(&self) -> Result<Memory>;

    /// Get swap usage.
    fn swap(&self) -> Result<Swap>;
}
