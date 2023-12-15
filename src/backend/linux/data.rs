//! Procfs object fetch.
use procfs::{Current, CurrentSI, ProcResult};

use crate::backend::{
    util::{RefreshRecord, Tick},
    BackendResult,
};

type ProcDataSource<T: Sized> = fn() -> ProcResult<T>;

fn pds_from_current<T: Current + Sized>() -> ProcDataSource<T> {
    T::current
}

fn pds_from_current_si<T: CurrentSI + Sized>() -> ProcDataSource<T> {
    T::current
}

/// Wrapper to fetch updated data from a `/proc` file.
pub(super) struct ProcFSData<T> {
    fetch: ProcDataSource<T>,
    pub current: Option<T>,
    pub previous: Option<T>,
    pub window: RefreshRecord,
}

impl<T: Current> ProcFSData<T> {
    /// Create a new data (fetches from a [Current] instance).
    pub(super) fn for_current(tick: &Tick) -> Self {
        ProcFSData {
            fetch: pds_from_current::<T>(),
            current: None,
            previous: None,
            window: RefreshRecord::with_tick(tick.clone()),
        }
    }
}

impl<T: CurrentSI> ProcFSData<T> {
    /// Create a new data (fetches from a [CurrentSI] instance).
    pub(super) fn for_curent_si(tick: &Tick) -> Self {
        ProcFSData {
            fetch: pds_from_current_si::<T>(),
            current: None,
            previous: None,
            window: RefreshRecord::with_tick(tick.clone()),
        }
    }
}

impl<T> ProcFSData<T> {
    /// Update the source if needed, based on the tick.
    pub(super) fn update_if_needed(&mut self) -> BackendResult<()> {
        if !self.window.is_current() {
            let cur = (self.fetch)()?;
            self.previous = self.current.replace(cur);
            self.window.update();
        }

        Ok(())
    }
}
