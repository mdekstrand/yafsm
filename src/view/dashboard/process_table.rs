use anyhow::Result;
use layout::Flex;
use log::*;
use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

use crate::{
    backend::error::BackendErrorFilter,
    model::{process::ProcessList, *},
    view::util::{fmt_bytes, fmt_duration, fmt_int_bytes},
};

type ColProc = fn(&dyn MonitorData, &Process) -> Result<String>;
type ColPredicate = fn(&dyn MonitorData) -> bool;

struct PTColumn {
    label: &'static str,
    constraint: Constraint,
    align: Alignment,
    sort_key: Option<ProcSortOrder>,
    ex_func: ColProc,
    active_pred: ColPredicate,
}

impl PTColumn {
    const fn new(label: &'static str) -> PTColumn {
        PTColumn {
            label,
            constraint: Constraint::Min(0),
            align: Alignment::Left,
            sort_key: None,
            ex_func: |_, _| Ok(String::new()),
            active_pred: |_| true,
        }
    }

    const fn width(self, width: u16) -> Self {
        PTColumn {
            constraint: Constraint::Length(width),
            ..self
        }
    }

    const fn min_width(self, width: u16) -> Self {
        PTColumn {
            constraint: Constraint::Min(width),
            ..self
        }
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

    const fn condition(self, active_pred: ColPredicate) -> Self {
        PTColumn {
            active_pred,
            ..self
        }
    }

    fn enabled(&self, state: &dyn MonitorData) -> bool {
        (self.active_pred)(state)
    }
}

static COLUMNS: &[PTColumn] = &[
    PTColumn::new("CPU%")
        .min_width(5)
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
        .condition(|state| state.backend().has_process_time())
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
        .min_width(20)
        .align(Alignment::Left)
        .extract(|state, proc| {
            let cmd = state.process_cmd_info(proc.pid);
            Ok(cmd
                .ok()
                .map(|c| c.cmdline.join(" "))
                .unwrap_or_else(|| format!("[{}]", proc.name)))
        }),
];

pub fn render_process_table<'b>(
    frame: &mut Frame,
    state: &MonitorState<'b>,
    area: Rect,
) -> Result<()> {
    let mut procs = if let Some(ps) = state.processes().acceptable_to_opt()? {
        ps
    } else {
        debug!("processes not available");
        return Ok(());
    };
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

fn render_headline<'b>(
    state: &MonitorState<'b>,
    procs: &ProcessList,
    frame: &mut Frame,
    area: Rect,
) -> Result<()> {
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

fn render_table<'b>(
    state: &MonitorState<'b>,
    procs: &ProcessList,
    frame: &mut Frame,
    area: Rect,
) -> Result<()> {
    debug!("proctbl: rendering {} processes in {:?}", procs.len(), area);
    let mut widths: Vec<_> = COLUMNS
        .iter()
        .filter(|c| c.enabled(state))
        .map(|c| c.constraint)
        .collect();

    let mut rows = Vec::with_capacity(procs.len());
    for proc in procs.iter() {
        rows.push(process_row(state, proc, &mut widths)?);
    }

    let header: Vec<_> = COLUMNS
        .iter()
        .filter(|c| c.enabled(state))
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
        .flex(Flex::Legacy)
        .highlight_symbol(">");
    frame.render_widget(table, area);
    Ok(())
}

fn process_row<'a, 'b>(
    state: &MonitorState<'b>,
    proc: &Process,
    widths: &mut [Constraint],
) -> Result<Row<'a>> {
    let mut cells = Vec::with_capacity(COLUMNS.len());
    for (i, col) in COLUMNS.iter().filter(|c| c.enabled(state)).enumerate() {
        if !col.enabled(state) {
            continue;
        }

        let text = (col.ex_func)(state, proc)?;
        let len = text.len();
        if let Constraint::Min(w) = widths[i] {
            if len <= 20 && len > (w as usize) {
                widths[i] = Constraint::Min(len as u16)
            }
        }
        let line = Line::from(text).alignment(col.align);
        cells.push(line);
    }
    Ok(Row::new(cells))
}
