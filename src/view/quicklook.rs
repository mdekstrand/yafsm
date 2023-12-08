//! Quick-look bar charts

use anyhow::Result;
use ratatui::prelude::*;
use sysinfo::{CpuExt, SystemExt};

use crate::SystemState;

use super::meter::Meter;

pub(super) fn render_quicklook(frame: &mut Frame, state: &SystemState, area: Rect) -> Result<()> {
    let cpu_usage = state.system.global_cpu_info().cpu_usage();
    let mem_tot = state.system.total_memory() as f32;
    let mem_usage = state.system.used_memory() as f32 / mem_tot;
    let mem_avail =
        (state.system.available_memory() as f32 - state.system.free_memory() as f32) / mem_tot;
    let swap_usage = state.system.used_swap() as f32 / state.system.total_swap() as f32;
    frame.render_widget(
        Meter::new("CPU").value(cpu_usage / 100.0),
        Rect {
            y: area.y + 1,
            height: 1,
            ..area
        },
    );
    frame.render_widget(
        Meter::new("MEM").value(mem_usage).second_value(mem_avail),
        Rect {
            y: area.y + 2,
            height: 1,
            ..area
        },
    );
    frame.render_widget(
        Meter::new("SWP").value(swap_usage),
        Rect {
            y: area.y + 3,
            height: 1,
            ..area
        },
    );
    Ok(())
}
