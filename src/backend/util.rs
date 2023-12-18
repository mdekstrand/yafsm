//! Backend utility functions and modules.

use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

/// Struct to record time between refreshes.
pub(super) struct RefreshRecord {
    /// The system tick.
    tick: Tick,
    /// The tick of the most recent refresh.
    last_tick: u64,
    /// The time of the most recent refresh.
    last_time: Instant,
    /// The time between the previous refresh and this one.
    duration: Duration,
}

impl RefreshRecord {
    /// Create a record without a reference to a system tick.
    pub fn new() -> RefreshRecord {
        RefreshRecord::with_tick(Tick::new())
    }

    pub fn with_tick(tick: Tick) -> RefreshRecord {
        RefreshRecord {
            tick,
            last_tick: 0,
            last_time: Instant::now(),
            duration: Duration::from_secs(1),
        }
    }

    /// Check if it is current (only works with a system tick).
    pub fn is_current(&self) -> bool {
        self.last_tick >= self.tick.current()
    }

    /// Get the last tick value.
    pub fn tick(&self) -> u64 {
        self.last_tick
    }

    /// Get the update window length.
    pub fn window_duration(&self) -> Duration {
        self.duration
    }

    /// Update the refresh window to mark a refresh at the current time.
    pub fn update(&mut self) {
        let now = Instant::now();
        self.duration = now.duration_since(self.last_time);
        self.last_tick = self.tick.current();
        self.last_time = now;
    }

    /// Normalize a value by the refresh window.  This takes a value in “units
    /// since last refresh” and converts it to “units per second”.
    pub fn norm_u64(&self, val: u64) -> u64 {
        window_norm_u64(val, self.duration)
    }

    /// Normalize a value by the refresh window.  This takes a value in “units
    /// since last refresh” and converts it to “units per second”.
    pub fn norm_f32(&self, val: f32) -> f32 {
        window_norm_f32(val, self.duration)
    }

    /// Normalize a value by the refresh window.  This takes a value in “units
    /// since last refresh” and converts it to “units per second”.
    pub fn norm_f64(&self, val: f64) -> f64 {
        window_norm_f64(val, self.duration)
    }
}

/// Normalize a value by the refresh window.  This takes a value in “units
/// since last refresh” and converts it to “units per second”.
pub(super) fn window_norm_u64(val: u64, dur: Duration) -> u64 {
    let time = dur.as_millis();
    (val as u128 * 1000 / time) as u64
}

/// Normalize a value by the refresh window.  This takes a value in “units
/// since last refresh” and converts it to “units per second”.
pub(super) fn window_norm_f32(val: f32, dur: Duration) -> f32 {
    let time = dur.as_secs_f32();
    val / time
}

/// Normalize a value by the refresh window.  This takes a value in “units
/// since last refresh” and converts it to “units per second”.
pub(super) fn window_norm_f64(val: f64, dur: Duration) -> f64 {
    let time = dur.as_secs_f64();
    val / time
}

/// A reference to a tick for tracking updates.  Cloning the tick
/// creates another refrence to the *same* tick.
#[derive(Clone)]
pub(super) struct Tick {
    tick: Rc<RefCell<u64>>,
}

impl Tick {
    pub fn new() -> Tick {
        Tick {
            tick: Rc::new(RefCell::new(0)),
        }
    }

    pub fn advance(&mut self) {
        *self.tick.borrow_mut() += 1
    }

    pub fn current(&self) -> u64 {
        *self.tick.borrow()
    }
}

/// Trait for computing differences between two observations.
pub(super) trait Diff {
    type Difference;

    fn diff(&self, previous: &Self) -> Self::Difference;
}

/// Trait for observations over a time window.
pub(super) trait WindowedObservation {
    /// Normalize the observation by the window length to yield measurements in
    /// units per second.
    fn normalize(&self, win: Duration) -> Self;
}
