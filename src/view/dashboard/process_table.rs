use anyhow::Result;
use log::*;
use ratatui::{
    prelude::*,
    widgets::{Cell, Row, Table},
};

use crate::{backend::MonitorBackend, model::*, view::util::fmt_bytes};

pub fn render_process_table<B>(frame: &mut Frame, state: &MonitorState<B>, area: Rect) -> Result<()>
where
    B: MonitorBackend,
{
    let mem = state.memory()?;
    let procs = state.processes()?;
    debug!("proctbl: rendering {} processes in {:?}", procs.len(), area);
    let mut rows = Vec::with_capacity(procs.len());
    for proc in &procs {
        rows.push(process_row(state, &mem, proc)?);
    }

    let table = Table::new(rows)
        .header(Row::new([
            Cell::from("CPU%"),
            Cell::from("MEM%"),
            Cell::from("VIRT"),
            Cell::from("RES"),
            Cell::from(Line::from("PID").alignment(Alignment::Right)),
            Cell::from(Line::from("USER").alignment(Alignment::Right)),
            Cell::from("S"),
            Cell::from("Command"),
        ]))
        .widths(&[
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .column_spacing(1)
        .highlight_symbol(">");
    frame.render_widget(table, area);

    Ok(())
}

fn process_row<'a, B>(state: &MonitorState<B>, mem: &Memory, proc: &Process<'a>) -> Result<Row<'a>>
where
    B: MonitorBackend,
{
    Ok(Row::new([
        Cell::from(format!("{:.1}", proc.cpu * 100.0)),
        Cell::from(format!(
            "{:.1}",
            proc.mem_rss as f32 * 100.0 / mem.total as f32
        )),
        Cell::from(fmt_bytes(proc.mem_virt)),
        Cell::from(fmt_bytes(proc.mem_rss)),
        Cell::from(Line::from(format!("{}", proc.pid)).alignment(Alignment::Right)),
        Cell::from(
            Line::from(
                proc.uid
                    .map(|u| state.lookup_user(u))
                    .transpose()?
                    .flatten()
                    .unwrap_or("??".into()),
            )
            .alignment(Alignment::Right),
        ),
        Cell::from(proc.status.to_string()),
        Cell::from(proc.cmd.join(" ")),
    ]))
}
