use anyhow::Result;
use friendly::duration;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::model::MonitorData;

pub(super) fn render_banner(frame: &mut Frame, state: &dyn MonitorData, area: Rect) -> Result<()> {
    let layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(area);

    let host = Paragraph::new(vec![Line::from(vec![
        Span::styled(state.hostname()?, Style::new().bold()),
        Span::raw(format!(" ({})", state.system_version()?)),
    ])])
    .alignment(Alignment::Left);
    let uptime = Paragraph::new(vec![Line::from(format!(
        "Uptime: {}",
        duration(state.uptime()?)
    ))])
    .alignment(Alignment::Right);

    frame.render_widget(host, layout[0]);
    frame.render_widget(uptime, layout[2]);

    Ok(())
}
