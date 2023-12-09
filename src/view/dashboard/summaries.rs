//! Summary box displays.

use anyhow::Result;
use ratatui::style::{Style, Stylize};

use crate::{
    model::MonitorData,
    view::widgets::infocols::{ICEntry, InfoCols},
};

pub fn cpu_summary(state: &dyn MonitorData) -> Result<InfoCols> {
    Ok(InfoCols::new().add(
        ICEntry::new("CPU")
            .pct(state.global_cpu()?.utilization * 100.0)
            .value_style(Style::new().bold()),
    ))
}

pub fn memory_summary(state: &dyn MonitorData) -> Result<InfoCols> {
    let mem = state.memory()?;
    Ok(InfoCols::new()
        .add(
            ICEntry::new("MEM")
                .pct(mem.used_frac() * 100.0)
                .value_style(Style::new().bold()),
        )
        .add_bytes("total", mem.total)
        .add_bytes("used", mem.used)
        .add_bytes("avail", mem.free + mem.freeable))
}

pub fn swap_summary(state: &dyn MonitorData) -> Result<InfoCols> {
    let swp = state.swap()?;
    Ok(InfoCols::new()
        .add(
            ICEntry::new("SWP")
                .pct(swp.used_frac() * 100.0)
                .value_style(Style::new().bold()),
        )
        .add_bytes("total", swp.total)
        .add_bytes("used", swp.used)
        .add_bytes("free", swp.free))
}

pub fn load_summary(state: &dyn MonitorData) -> Result<InfoCols> {
    let load = state.load_avg()?;
    Ok(InfoCols::new()
        .add_str("LOAD", format!("{}core", state.cpu_count()?))
        .add_value("1min", load.one)
        .add_value("5min", load.five)
        .add_value("15min", load.fifteen))
}
