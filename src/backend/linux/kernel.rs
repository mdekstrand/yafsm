use procfs::{CurrentSI, KernelStats};

use crate::backend::util::{RefreshRecord, RefreshableSource, Tick};

pub(super) struct Stats {
    prev_times: Option<KernelStats>,
    cur_times: Option<KernelStats>,
    window: RefreshRecord,
}

impl Stats {
    pub(super) fn new(tick: Tick) -> Stats {
        Stats {
            prev_times: None,
            cur_times: None,
            window: RefreshRecord::with_tick(tick),
        }
    }
}

impl RefreshableSource for Stats {
    fn refresh_record(&mut self) -> &mut RefreshRecord {
        &mut self.window
    }

    fn update(&mut self) -> crate::backend::BackendResult<()> {
        let times = KernelStats::current()?;
        self.prev_times = self.cur_times.replace(times);
        Ok(())
    }
}
