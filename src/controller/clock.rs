use std::time::{Duration, SystemTime};

const REFRESH_TOL: Duration = Duration::from_millis(50);

/// Clock to manage polling and refresh rates.  This thing has a weird interface that
/// is closely intertwined with how it is used by [run_event_loop].
pub struct Clock {
    last_refresh: SystemTime,
    now: SystemTime,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            last_refresh: SystemTime::UNIX_EPOCH,
            now: SystemTime::now(),
        }
    }

    /// Update the clock's current time.
    pub fn update_now(&mut self) {
        self.now = SystemTime::now()
    }

    pub fn next_wait(&mut self) -> Duration {
        let st = self
            .now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);
        let millis = st.subsec_millis();
        Duration::from_millis(1000 - millis as u64)
    }

    pub fn mark_refresh(&mut self) {
        // we do *not* update — mark when we started the refresh
        self.last_refresh = self.now
    }

    pub fn want_refresh(&mut self, period: Duration) -> bool {
        // need to update now, this will be called later
        self.update_now();
        if let Ok(diff) = self.now.duration_since(self.last_refresh) {
            diff >= period - REFRESH_TOL
        } else {
            // time went backwards
            true
        }
    }
}
