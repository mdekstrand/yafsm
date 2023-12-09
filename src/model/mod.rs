//! Access to the different monitors.
use std::time::Duration;

use anyhow::*;

pub mod cpu;
pub mod memory;
pub mod options;
pub mod source;
pub mod swap;

pub use cpu::CPU;
pub use memory::Memory;
pub use options::Options;
pub use swap::Swap;

use crate::backend::MonitorBackend;

use self::source::{SystemInfo, SystemResources};

/// Interface for data monitor sources.
///
/// This is defined as a trait so the monitor state can be object-safe, where that might
/// be helpful.  It also has methods that are somewhat duplicative of [MonitorBackend],
/// but many of them handle checking whether that feature should be enabled.
pub trait MonitorData: SystemInfo + SystemResources {
    fn backend(&self) -> &dyn MonitorBackend;
}

/// Container for system monitor state.
pub struct MonitorState<B: MonitorBackend> {
    pub options: Options,
    pub backend: B,
}

impl<B> MonitorState<B>
where
    B: MonitorBackend,
{
    pub fn create(options: Options, backend: B) -> Result<MonitorState<B>> {
        Ok(MonitorState { options, backend })
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.backend.update(&self.options)
    }
}

impl<B> MonitorData for MonitorState<B>
where
    B: MonitorBackend,
{
    fn backend(&self) -> &dyn MonitorBackend {
        &self.backend
    }
}

impl<B> SystemInfo for MonitorState<B>
where
    B: MonitorBackend,
{
    fn hostname(&self) -> Result<String> {
        self.backend.hostname()
    }

    fn system_version(&self) -> Result<String> {
        self.backend.system_version()
    }

    fn uptime(&self) -> Result<Duration> {
        self.backend.uptime()
    }
}

impl<B> SystemResources for MonitorState<B>
where
    B: MonitorBackend,
{
    fn global_cpu(&self) -> Result<CPU> {
        self.backend.global_cpu()
    }

    fn memory(&self) -> Result<Memory> {
        self.backend.memory()
    }

    fn swap(&self) -> Result<Swap> {
        self.backend.swap()
    }
}
