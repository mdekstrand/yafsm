//! Access to the different monitors.
use anyhow::*;

use crate::backend::sysmon::initialize;

pub mod cpu;
pub mod memory;
pub mod options;
pub mod swap;

pub use cpu::CPU;
pub use memory::Memory;
pub use options::Options;
pub use swap::Swap;

use crate::backend::MonitorBackend;

/// Interface for data monitor sources.
///
/// This is defined as a trait so the monitor state can be object-safe, where that might
/// be helpful.
pub trait MonitorData {
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
        let mut system = initialize()?;

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
