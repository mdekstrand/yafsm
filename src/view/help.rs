//! Help display

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, Flex, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Clear, List, ListItem, Padding};
use ratatui::Frame;

use crate::model::MonitorState;

pub fn render_help<'b>(
    frame: &mut Frame,
    _state: &MonitorState<'b>,
    bindings: &[(KeyCode, &str)],
) -> Result<()> {
    let block = Block::bordered()
        .title("Help")
        .title_style(Style::new().fg(Color::Blue))
        .title_alignment(Alignment::Center)
        .border_style(Style::new().fg(Color::Blue))
        .padding(Padding::horizontal(1));

    let mut lines = Vec::<ListItem>::new();
    for (kc, desc) in bindings {
        match kc {
            KeyCode::Null => {
                if !lines.is_empty() {
                    lines.push(ListItem::new(""));
                }
                lines.push(ListItem::new(desc.to_string()).style(Style::new().bold()));
            }
            KeyCode::Char(c) => lines.push(ListItem::new(format!("{}   {}", c, desc))),
            _ => (),
        }
    }

    let height: usize = lines.iter().map(|li| li.height()).sum();
    let width: usize = lines.iter().map(|li| li.width()).max().unwrap_or_default();

    let help = List::new(lines).block(block);

    let v_layout = Layout::vertical([Constraint::Length(height as u16 + 2)])
        .flex(Flex::Center)
        .areas::<1>(frame.area());
    let h_layout = Layout::horizontal([Constraint::Length(width as u16 + 4)])
        .flex(Flex::Center)
        .areas::<1>(v_layout[0]);
    frame.render_widget(Clear, h_layout[0]);
    frame.render_widget(help, h_layout[0]);

    Ok(())
}
