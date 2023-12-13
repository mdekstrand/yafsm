//! Backend utility functions and modules.

use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use super::error::BackendResult;

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
        self.last_tick < self.tick.current()
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
        let time = self.duration.as_millis();
        (val as u128 * 1000 / time) as u64
    }

    /// Normalize a value by the refresh window.  This takes a value in “units
    /// since last refresh” and converts it to “units per second”.
    pub fn norm_f32(&self, val: f32) -> f32 {
        let time = self.duration.as_secs_f32();
        val / time
    }

    /// Normalize a value by the refresh window.  This takes a value in “units
    /// since last refresh” and converts it to “units per second”.
    pub fn norm_f64(&self, val: f64) -> f64 {
        let time = self.duration.as_secs_f64();
        val / time
    }
}

/// A reference to a tick for tracking updates.  Cloning the tick
/// creates another refrence to the *same* tick.
#[derive(Clone)]
pub struct Tick {
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

pub trait RefreshableSource {
    /// Get the source's refresh record (only used internally).
    fn refresh_record(&mut self) -> &mut RefreshRecord;

    /// Update the source.
    fn update(&mut self) -> BackendResult<()>;

    /// Update the source if needed, based on the tick.
    fn update_if_needed(&mut self) -> BackendResult<()> {
        if !self.refresh_record().is_current() {
            self.update()
        } else {
            Ok(())
        }
    }
}
