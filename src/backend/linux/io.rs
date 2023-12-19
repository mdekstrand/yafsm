use std::{collections::HashMap, time::Duration};

use procfs::{DiskStat, DiskStats};

use super::data::ProcFSWrapper;
use crate::{
    backend::{
        util::{window_norm_u64, Diff, WindowedObservation},
        BackendError, BackendResult,
    },
    model::DiskIO,
};

const BYTES_PER_SECTOR: u64 = 512;

impl Diff for DiskIO {
    type Difference = Self;

    fn diff(&self, previous: &Self) -> Self::Difference {
        DiskIO {
            name: self.name.clone(),
            rx_bytes: self.rx_bytes - previous.rx_bytes,
            tx_bytes: self.tx_bytes - previous.tx_bytes,
        }
    }
}

impl From<&DiskStat> for DiskIO {
    fn from(d: &DiskStat) -> Self {
        DiskIO {
            name: d.name.clone(),
            rx_bytes: d.sectors_read * BYTES_PER_SECTOR,
            tx_bytes: d.sectors_written * BYTES_PER_SECTOR,
        }
    }
}

impl WindowedObservation for DiskIO {
    fn normalize(&self, win: Duration) -> Self {
        DiskIO {
            name: self.name.clone(),
            rx_bytes: window_norm_u64(self.rx_bytes, win),
            tx_bytes: window_norm_u64(self.tx_bytes, win),
        }
    }
}

impl ProcFSWrapper<DiskStats> {
    pub(super) fn disk_stats(&self) -> BackendResult<Vec<DiskIO>> {
        let data = self.data()?;
        let DiskStats(cur) = data.current.as_ref().ok_or(BackendError::NotAvailable)?;
        let prev = data.previous.as_ref().map(|DiskStats(p)| p);
        let prev: Option<HashMap<_, _>> =
            prev.map(|v| v.iter().map(|d| (d.name.clone(), d.clone())).collect());
        Ok(cur
            .iter()
            .map(|d| {
                if let Some(p) = prev.as_ref().and_then(|h| h.get(&d.name)) {
                    let d: DiskIO = d.into();
                    let p: DiskIO = p.into();
                    d.diff(&p).normalize(data.window.window_duration())
                } else {
                    d.into()
                }
            })
            .collect())
    }
}
