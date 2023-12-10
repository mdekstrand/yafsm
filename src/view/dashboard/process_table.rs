use anyhow::Result;
use log::*;
use ratatui::{
    layout::SegmentSize,
    prelude::*,
    widgets::{Cell, Row, Table},
};

use crate::{
    backend::MonitorBackend,
    model::*,
    view::util::{fmt_bytes, fmt_duration, fmt_int_bytes},
};

pub fn render_process_table<B>(frame: &mut Frame, state: &MonitorState<B>, area: Rect) -> Result<()>
where
    B: MonitorBackend,
{
    let mem = state.memory()?;
    let mut procs = state.processes()?;
    procs.sort_by(|p1, p2| p2.cpu_util.total_cmp(&p1.cpu_util));
    debug!("proctbl: rendering {} processes in {:?}", procs.len(), area);
    let mut rows = Vec::with_capacity(procs.len());
    for proc in &procs {
        rows.push(process_row(state, &mem, proc)?);
    }

    let table = Table::new(
        rows,
        &[
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(20),
        ],
    )
    .header(Row::new([
        Cell::from("CPU%"),
        Cell::from("MEM%"),
        Cell::from("VIRT"),
        Cell::from("RES"),
        Cell::from(Line::from("PID").alignment(Alignment::Right)),
        Cell::from(Line::from("USER").alignment(Alignment::Right)),
        Cell::from(Line::from("TIME").alignment(Alignment::Right)),
        Cell::from("S"),
        Cell::from(Line::from("R/s").alignment(Alignment::Right)),
        Cell::from(Line::from("W/s").alignment(Alignment::Right)),
        Cell::from("Command"),
    ]))
    .column_spacing(1)
    .segment_size(SegmentSize::LastTakesRemainder)
    .highlight_symbol(">");
    frame.render_widget(table, area);

    Ok(())
}

fn process_row<'a, B>(state: &MonitorState<B>, mem: &Memory, proc: &Process) -> Result<Row<'a>>
where
    B: MonitorBackend,
{
    let cmd = state.process_details(proc.pid).ok();
    Ok(Row::new([
        Cell::from(format!("{:.1}", proc.cpu_util * 100.0)),
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
        Cell::from(
            Line::from(proc.cpu_time.map(fmt_duration).unwrap_or_default())
                .alignment(Alignment::Right),
        ),
        Cell::from(proc.status.to_string()),
        Cell::from(
            Line::from(proc.io_read.map(fmt_int_bytes).unwrap_or_default())
                .alignment(Alignment::Right),
        ),
        Cell::from(
            Line::from(proc.io_write.map(fmt_int_bytes).unwrap_or_default())
                .alignment(Alignment::Right),
        ),
        Cell::from(
            cmd.map(|c| c.cmdline.join(" "))
                .unwrap_or_else(|| format!("[{}]", proc.name)),
        ),
    ]))
}
