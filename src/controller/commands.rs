//! Key binding actions.
use crate::model::MonitorState;
use crossterm::event::KeyCode;

type CommandAction = for<'a> fn(&mut MonitorState<'a>) -> ();

const fn kc(c: char) -> KeyCode {
    KeyCode::Char(c)
}

fn kc_quit(state: &mut MonitorState<'_>) {
    state.running = false;
}

fn kc_sort_auto(state: &mut MonitorState<'_>) {
    state.proc_sort = None
}

fn kc_sort_cpu(state: &mut MonitorState<'_>) {
    state.proc_sort = Some(crate::model::ProcSortOrder::CPU)
}

fn kc_sort_memory(state: &mut MonitorState<'_>) {
    state.proc_sort = Some(crate::model::ProcSortOrder::Memory)
}

fn kc_sort_io(state: &mut MonitorState<'_>) {
    state.proc_sort = Some(crate::model::ProcSortOrder::IO)
}

fn kc_sort_time(state: &mut MonitorState<'_>) {
    state.proc_sort = Some(crate::model::ProcSortOrder::Time)
}

pub static KEY_BINDINGS: &[(KeyCode, &str, CommandAction)] = &[
    (kc('q'), "quit", kc_quit),
    (kc('a'), "sort automatically", kc_sort_auto),
    (kc('c'), "sort by CPU", kc_sort_cpu),
    (kc('m'), "sort by memory", kc_sort_memory),
    (kc('i'), "sort by IO", kc_sort_io),
    (kc('t'), "sort by time", kc_sort_time),
];
