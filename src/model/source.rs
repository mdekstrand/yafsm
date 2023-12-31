use super::{load::SystemPressure, process::ProcessList, *};
use crate::backend::BackendResult as Result;

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

    /// Get pressure stall info.
    fn pressure(&self) -> Result<SystemPressure>;
}

pub trait RunningProcesses {
    /// Get the running processes.
    fn processes(&self) -> Result<ProcessList>;

    /// Get command information for a process.
    fn process_cmd_info(&self, pid: u32) -> Result<ProcessCommandInfo>;
}

pub trait NetworkInfo {
    /// Get the networks.
    fn networks(&self) -> Result<Vec<NetworkStats>>;
}

pub trait StorageInfo {
    /// Get the disks.
    fn disk_io(&self) -> Result<Vec<DiskIO>>;

    /// Get the filesystems.
    fn filesystems(&self) -> Result<Vec<Filesystem>>;
}

pub trait GPUInfo {
    /// Get the GPUs.
    fn gpus(&self) -> Result<Vec<GPUStats>>;
}
