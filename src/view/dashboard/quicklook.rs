//! Quick-look bar charts

use anyhow::Result;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::backend::error::BackendErrorFilter;
use crate::model::cpu::ExtendedCPU;
use crate::model::MonitorData;
use crate::view::util::level_color;
use crate::view::widgets::meter::Meter;

pub fn render_quicklook(frame: &mut Frame, state: &dyn MonitorData, area: Rect) -> Result<()> {
    if let Some(cpu) = state.global_cpu().acceptable_to_opt()? {
        frame.render_widget(
            if let ExtendedCPU::Linux(cpu) = cpu.extended {
                Meter::new("CPU")
                    .value(cpu.user, Color::Green)
                    .value(cpu.system, Color::Red)
                    .value(cpu.iowait, Color::DarkGray)
            } else {
                Meter::new("CPU").value(cpu.utilization, level_color(cpu.utilization))
            },
            Rect {
                y: area.y + 1,
                height: 1,
                ..area
            },
        );
    } else {
        frame.render_widget(
            Paragraph::new(vec![Line::from(
                Span::from("CPU unavailable").fg(Color::LightRed),
            )]),
            Rect {
                y: area.y + 1,
                height: 1,
                ..area
            },
        );
    }

    if let Some(mem) = state.memory().acceptable_to_opt()? {
        frame.render_widget(
            Meter::new("MEM")
                .value(mem.used_frac(), level_color(mem.used_frac()))
                .value(mem.shared_frac(), Color::Cyan)
                .value(mem.freeable_frac(), Color::DarkGray),
            Rect {
                y: area.y + 2,
                height: 1,
                ..area
            },
        );
    } else {
        frame.render_widget(
            Paragraph::new(vec![Line::from(
                Span::from("Memory unavailable").fg(Color::LightRed),
            )]),
            Rect {
                y: area.y + 2,
                height: 1,
                ..area
            },
        );
    }

    if let Some(swap) = state.swap().acceptable_to_opt()? {
        frame.render_widget(
            Meter::new("SWP").value(swap.used_frac(), level_color(swap.used_frac())),
            Rect {
                y: area.y + 3,
                height: 1,
                ..area
            },
        );
    } else {
        frame.render_widget(
            Paragraph::new(vec![Line::from(
                Span::from("Swap unavailable").fg(Color::LightRed),
            )]),
            Rect {
                y: area.y + 3,
                height: 1,
                ..area
            },
        );
    }

    Ok(())
}
