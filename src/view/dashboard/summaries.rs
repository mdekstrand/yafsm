//! Summary box displays.

use anyhow::Result;
use ratatui::style::{Color, Style, Stylize};

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
    let ncpus = state.cpu_count()? as f32;
    let load = state.load_avg()?;
    Ok(InfoCols::new()
        .add(ICEntry::new("LOAD").string(format!("{}core", state.cpu_count()?)))
        .add(
            ICEntry::new("1min")
                .value(load.one)
                .value_style(Style::new().fg(if load.one >= ncpus {
                    Color::Red
                } else if load.one >= ncpus * 0.5 {
                    Color::Magenta
                } else {
                    Color::White
                })),
        )
        .add(
            ICEntry::new("5min")
                .value(load.five)
                .value_style(Style::new().fg(if load.five >= ncpus {
                    Color::Red
                } else if load.five >= ncpus * 0.5 {
                    Color::Magenta
                } else {
                    Color::White
                })),
        )
        .add(
            ICEntry::new("15min")
                .value(load.fifteen)
                .value_style(Style::new().fg(if load.fifteen >= ncpus {
                    Color::Red
                } else if load.fifteen >= ncpus * 0.5 {
                    Color::Magenta
                } else {
                    Color::White
                })),
        ))
}
