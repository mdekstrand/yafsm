//! Backend utility functions and modules.

use std::time::{Duration, Instant};

/// Struct to record time between refreshes.
pub(super) struct RefreshRecord {
    /// The time of the most recent refresh.
    time: Instant,
    /// The time between the previous refresh and this one.
    duration: Duration,
}

impl RefreshRecord {
    pub fn new() -> RefreshRecord {
        RefreshRecord {
            time: Instant::now(),
            duration: Duration::from_secs(1),
        }
    }

    /// Update the refresh window to mark a refresh at the current time.
    pub fn update(&mut self) {
        let now = Instant::now();
        self.duration = now.duration_since(self.time);
        self.time = now;
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
