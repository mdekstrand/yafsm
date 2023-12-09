//! Quick-look bar charts

use anyhow::Result;
use ratatui::prelude::*;

use crate::model::MonitorData;

use super::meter::Meter;
use super::util::level_color;

pub(super) fn render_quicklook(
    frame: &mut Frame,
    state: &dyn MonitorData,
    area: Rect,
) -> Result<()> {
    let cpu = state.global_cpu()?;
    let mem = state.memory()?;
    let swap = state.swap()?;
    frame.render_widget(
        Meter::new("CPU").value(cpu.utilization, level_color(cpu.utilization)),
        Rect {
            y: area.y + 1,
            height: 1,
            ..area
        },
    );
    frame.render_widget(
        Meter::new("MEM")
            .value(mem.used_frac(), level_color(mem.used_frac()))
            .value(mem.freeable_frac(), Color::Cyan),
        Rect {
            y: area.y + 2,
            height: 1,
            ..area
        },
    );
    frame.render_widget(
        Meter::new("SWP").value(swap.used_frac(), level_color(swap.used_frac())),
        Rect {
            y: area.y + 3,
            height: 1,
            ..area
        },
    );
    Ok(())
}
