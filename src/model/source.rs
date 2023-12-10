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
    /// Get the number of CPUs.
    fn cpu_count(&self) -> Result<u32>;

    /// Get CPU utilization.
    fn global_cpu(&self) -> Result<CPU>;

    /// Get memory usage.
    fn memory(&self) -> Result<Memory>;

    /// Get swap usage.
    fn swap(&self) -> Result<Swap>;

    /// Get the system load average.
    fn load_avg(&self) -> Result<LoadAvg>;
}

pub trait RunningProcesses {
    /// Get the running processes.
    fn processes(&self) -> Result<ProcessList>;

    /// Get command information for a process.
    fn process_details(&self, pid: u32) -> Result<ProcessDetails>;
}
