//! Help display

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListItem};
use ratatui::Frame;

use crate::model::MonitorState;

pub fn render_help<'b, A>(
    frame: &mut Frame,
    _state: &MonitorState<'b>,
    bindings: &[(KeyCode, &str, A)],
) -> Result<()> {
    let block = Block::bordered().title("Help");

    let mut lines = Vec::<ListItem>::new();
    for (kc, desc, _action) in bindings {
        match kc {
            KeyCode::Null => {
                if !lines.is_empty() {
                    lines.push(ListItem::new(""));
                }
                lines.push(ListItem::new(desc.to_uppercase()).style(Style::new().bold()));
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
    let h_layout = Layout::horizontal([Constraint::Length(width as u16 + 2)])
        .flex(Flex::Center)
        .areas::<1>(v_layout[0]);
    frame.render_widget(help, h_layout[0]);

    Ok(())
}
