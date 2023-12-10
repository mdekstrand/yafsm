use anyhow::Result;
use log::*;
use ratatui::{
    layout::SegmentSize,
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

use crate::{
    backend::MonitorBackend,
    model::{process::ProcessList, *},
    view::util::{fmt_bytes, fmt_duration, fmt_int_bytes},
};

type ColProc = fn(&dyn MonitorData, &Process) -> Result<String>;

struct PTColumn {
    label: &'static str,
    width: u16,
    align: Alignment,
    sort_key: Option<ProcSortOrder>,
    ex_func: ColProc,
}

impl PTColumn {
    const fn new(label: &'static str) -> PTColumn {
        PTColumn {
            label,
            width: 0,
            align: Alignment::Left,
            sort_key: None,
            ex_func: |_, _| Ok(String::new()),
        }
    }

    const fn width(self, width: u16) -> Self {
        PTColumn { width, ..self }
    }

    const fn align(self, align: Alignment) -> Self {
        PTColumn { align, ..self }
    }

    const fn sort(self, key: ProcSortOrder) -> Self {
        PTColumn {
            sort_key: Some(key),
            ..self
        }
    }

    const fn extract(self, ex_func: ColProc) -> Self {
        PTColumn { ex_func, ..self }
    }
}

static COLUMNS: &[PTColumn] = &[
    PTColumn::new("CPU%")
        .width(4)
        .align(Alignment::Right)
        .sort(ProcSortOrder::CPU)
        .extract(|_, proc| Ok(format!("{:.1}", proc.cpu_util * 100.0))),
    PTColumn::new("MEM%")
        .width(5)
        .align(Alignment::Right)
        .sort(ProcSortOrder::Memory)
        .extract(|_, proc| Ok(format!("{:.1}", proc.mem_util * 100.0))),
    PTColumn::new("VIRT")
        .width(5)
        .align(Alignment::Right)
        .extract(|_, proc| Ok(fmt_bytes(proc.mem_virt))),
    PTColumn::new("RES")
        .width(6)
        .align(Alignment::Right)
        .extract(|_, proc| Ok(fmt_bytes(proc.mem_rss))),
    PTColumn::new("PID")
        .width(6)
        .align(Alignment::Right)
        .extract(|_, proc| Ok(format!("{}", proc.pid))),
    PTColumn::new("USER")
        .width(8)
        .align(Alignment::Right)
        .extract(|state, proc| {
            Ok(proc
                .uid
                .map(|u| state.lookup_user(u))
                .transpose()?
                .flatten()
                .unwrap_or("??".into()))
        }),
    PTColumn::new("TIME")
        .width(5)
        .align(Alignment::Right)
        .sort(ProcSortOrder::Time)
        .extract(|_, proc| Ok(proc.cpu_time.map(fmt_duration).unwrap_or_default())),
    PTColumn::new("S")
        .width(1)
        .align(Alignment::Center)
        .extract(|_, proc| Ok(proc.status.to_string())),
    PTColumn::new("R/s")
        .width(5)
        .align(Alignment::Right)
        .sort(ProcSortOrder::IO)
        .extract(|_, proc| Ok(proc.io_read.map(fmt_int_bytes).unwrap_or_default())),
    PTColumn::new("W/s")
        .width(5)
        .align(Alignment::Right)
        .sort(ProcSortOrder::IO)
        .extract(|_, proc| Ok(proc.io_write.map(fmt_int_bytes).unwrap_or_default())),
    PTColumn::new("Command")
        .width(0)
        .align(Alignment::Left)
        .extract(|state, proc| {
            let cmd = state.process_details(proc.pid);
            Ok(cmd
                .ok()
                .map(|c| c.cmdline.join(" "))
                .unwrap_or_else(|| format!("[{}]", proc.name)))
        }),
];

pub fn render_process_table<B>(frame: &mut Frame, state: &MonitorState<B>, area: Rect) -> Result<()>
where
    B: MonitorBackend,
{
    let mut procs = state.processes()?;
    procs.sort();

    let layout = Layout::new(
        Direction::Vertical,
        &[
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(5),
        ],
    )
    .split(area);

    render_headline(state, &procs, frame, layout[0])?;
    render_table(state, &procs, frame, layout[2])?;

    Ok(())
}

fn render_headline<B>(
    state: &MonitorState<B>,
    procs: &ProcessList,
    frame: &mut Frame,
    area: Rect,
) -> Result<()>
where
    B: MonitorBackend,
{
    let counts = procs.counts();
    let hl = Line::from(vec![
        Span::from("TASKS").bold(),
        Span::from(format!(
            " {} run, {} slp, {} oth",
            counts.running, counts.sleeping, counts.other
        )),
        Span::from(" sorted"),
        Span::from(if state.proc_sort.is_none() {
            " automatically"
        } else {
            ""
        }),
        Span::from(" by "),
        Span::from(match procs.active_sort_order() {
            ProcSortOrder::CPU => "CPU usage",
            ProcSortOrder::Memory => "memory",
            ProcSortOrder::IO => "total I/O",
            ProcSortOrder::Time => "time",
        }),
    ]);
    let hl = Paragraph::new(vec![hl]);

    frame.render_widget(hl, area);
    Ok(())
}

fn render_table<B>(
    state: &MonitorState<B>,
    procs: &ProcessList,
    frame: &mut Frame,
    area: Rect,
) -> Result<()>
where
    B: MonitorBackend,
{
    debug!("proctbl: rendering {} processes in {:?}", procs.len(), area);
    let mut rows = Vec::with_capacity(procs.len());
    for proc in procs.iter() {
        rows.push(process_row(state, proc)?);
    }

    let widths: Vec<_> = COLUMNS
        .iter()
        .map(|c| {
            if c.width > 0 {
                Constraint::Length(c.width)
            } else {
                Constraint::Min(20)
            }
        })
        .collect();
    let header: Vec<_> = COLUMNS
        .iter()
        .map(|c| {
            let span = Span::from(c.label);
            let span = match c.sort_key {
                Some(s) if s == procs.active_sort_order() => span.bold().underlined(),
                _ => span,
            };
            Cell::from(Line::from(span).alignment(c.align))
        })
        .collect();
    let table = Table::new(rows, &widths)
        .header(Row::new(header))
        .column_spacing(1)
        .segment_size(SegmentSize::LastTakesRemainder)
        .highlight_symbol(">");
    frame.render_widget(table, area);
    Ok(())
}

fn process_row<'a, B>(state: &MonitorState<B>, proc: &Process) -> Result<Row<'a>>
where
    B: MonitorBackend,
{
    let mut cells = Vec::with_capacity(COLUMNS.len());
    for col in COLUMNS {
        let text = (col.ex_func)(state, proc)?;
        let line = Line::from(text).alignment(col.align);
        cells.push(line);
    }
    Ok(Row::new(cells))
}
