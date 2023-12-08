use std::time::Duration;

use anyhow::Result;
use friendly::duration;
use ratatui::{prelude::*, widgets::Paragraph};
use sysinfo::SystemExt;

use crate::SystemMonitor;

pub(super) fn render_banner(frame: &mut Frame, state: &SystemMonitor, area: Rect) -> Result<()> {
    let layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(area);

    let host = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            state.system.host_name().unwrap_or("unnamed".into()),
            Style::new().bold(),
        ),
        Span::raw(format!(
            " ({} {})",
            state.system.distribution_id(),
            state.system.os_version().unwrap_or_default()
        )),
    ])])
    .alignment(Alignment::Left);
    let uptime = Paragraph::new(vec![Line::from(format!(
        "Uptime: {}",
        duration(Duration::from_secs(state.system.uptime()))
    ))])
    .alignment(Alignment::Right);

    frame.render_widget(host, layout[0]);
    frame.render_widget(uptime, layout[2]);

    Ok(())
}
