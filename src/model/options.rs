//! Options for models and backend state.
use std::time::Duration;

/// Struct containing the options for the system viewer.  These are initialized from
/// the command line and defaults, and some can be modified interactively.
pub struct Options {
    /// Refresh interval.
    pub refresh: Duration,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            refresh: Duration::from_millis(2500),
        }
    }
}
