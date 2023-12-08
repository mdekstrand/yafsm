//! Quick-look bar charts

use anyhow::Result;
use ratatui::prelude::*;

use crate::model::MonitorData;

use super::meter::Meter;

pub(super) fn render_quicklook(
    frame: &mut Frame,
    state: &dyn MonitorData,
    area: Rect,
) -> Result<()> {
    let cpu = state.global_cpu()?;
    let mem = state.memory()?;
    let swap = state.swap()?;
    frame.render_widget(
        Meter::new("CPU").value(cpu.utilization),
        Rect {
            y: area.y + 1,
            height: 1,
            ..area
        },
    );
    frame.render_widget(
        Meter::new("MEM")
            .value(mem.used_frac())
            .second_value(mem.freeable_frac()),
        Rect {
            y: area.y + 2,
            height: 1,
            ..area
        },
    );
    frame.render_widget(
        Meter::new("SWP").value(swap.used_frac()),
        Rect {
            y: area.y + 3,
            height: 1,
            ..area
        },
    );
    Ok(())
}
