//! Procfs object fetch.
use std::cell::{Ref, RefCell};
use std::fmt::Debug;

use log::*;
use procfs::{Current, CurrentSI, ProcResult};

use crate::backend::BackendError;
use crate::backend::{
    util::{RefreshRecord, Tick},
    BackendResult,
};

type ProcDataSource<T> = fn() -> ProcResult<T>;

fn pds_from_current<T: Current + Sized>() -> ProcDataSource<T> {
    T::current
}

fn pds_from_current_si<T: CurrentSI + Sized>() -> ProcDataSource<T> {
    T::current
}

/// Wrapper to fetch updated data from a `/proc` file.
pub(super) struct ProcFSWrapper<T> {
    fetch: ProcDataSource<T>,
    state: RefCell<ProcFSData<T>>,
}

pub(super) struct ProcFSData<T> {
    pub current: Option<T>,
    pub previous: Option<T>,
    pub window: RefreshRecord,
}

impl<T> ProcFSWrapper<T> {
    fn new(fetch: ProcDataSource<T>, tick: &Tick) -> Self {
        ProcFSWrapper {
            fetch,
            state: RefCell::new(ProcFSData {
                current: None,
                previous: None,
                window: RefreshRecord::with_tick(tick.clone()),
            }),
        }
    }
}

impl<T: Current> ProcFSWrapper<T> {
    /// Create a new data (fetches from a [Current] instance).
    pub(super) fn for_current(tick: &Tick) -> Self {
        ProcFSWrapper::new(pds_from_current::<T>(), tick)
    }
}

impl<T: CurrentSI> ProcFSWrapper<T> {
    /// Create a new data (fetches from a [CurrentSI] instance).
    pub(super) fn for_curent_si(tick: &Tick) -> Self {
        ProcFSWrapper::new(pds_from_current_si::<T>(), tick)
    }
}

impl<T: Debug> ProcFSWrapper<T> {
    /// Get the current state, updating if necessary.
    pub(super) fn data<'a>(&'a self) -> BackendResult<Ref<'a, ProcFSData<T>>> {
        let mut state = self.state.borrow_mut();
        if !state.window.is_current() {
            let cur = (self.fetch)()?;
            state.window.update();
            trace!("tick {} fetched data: {:#?}", state.window.tick(), cur);
            state.previous = state.current.replace(cur);
        }
        drop(state);

        Ok(self.state.borrow())
    }

    /// Get the current data, updating if necessary.
    pub(super) fn current<'a>(&'a self) -> BackendResult<Ref<'a, T>> {
        self.data().and_then(|c| {
            Ref::filter_map(c, |s| s.current.as_ref()).map_err(|_| BackendError::NotAvailable)
        })
    }
}
