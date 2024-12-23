//! Key binding actions.
use crate::model::MonitorState;
use crossterm::event::KeyCode;

pub type CommandAction<T> = for<'a> fn(&mut MonitorState<'a>) -> T;
pub type KeyBindingSet<'d, T> = [(KeyCode, &'d str, CommandAction<T>)];

pub fn dispatch_key<'a, 'd, T: Default>(
    code: KeyCode,
    bindings: &KeyBindingSet<'d, T>,
    state: &mut MonitorState<'a>,
) -> T {
    for (kc, _desc, action) in bindings {
        if code == *kc {
            return action(state);
        }
    }
    T::default()
}

/// Convenience function for making character keycodes.
pub const fn kc(c: char) -> KeyCode {
    KeyCode::Char(c)
}
/// No-op key command.
pub fn kc_nop<T: Default>(_state: &mut MonitorState<'_>) -> T {
    T::default()
}
