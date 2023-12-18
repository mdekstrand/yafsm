use std::collections::HashMap;

use itertools::Itertools;
use log::*;
use procfs::net::*;

use super::data::{ProcFSData, ProcFSWrapper};
use crate::backend::{
    util::{window_norm_u64, Diff, WindowedObservation},
    BackendError, BackendResult,
};

impl Diff for DeviceStatus {
    type Difference = Self;

    fn diff(&self, previous: &Self) -> Self::Difference {
        DeviceStatus {
            name: self.name.clone(),
            recv_bytes: self.recv_bytes - previous.recv_bytes,
            recv_packets: self.recv_packets - previous.recv_packets,
            recv_errs: self.recv_errs - previous.recv_errs,
            recv_drop: self.recv_drop - previous.recv_drop,
            recv_fifo: self.recv_fifo - previous.recv_fifo,
            recv_frame: self.recv_frame - previous.recv_frame,
            recv_compressed: self.recv_compressed - previous.recv_compressed,
            recv_multicast: self.recv_multicast - previous.recv_multicast,
            sent_bytes: self.sent_bytes - previous.sent_bytes,
            sent_packets: self.sent_packets - previous.sent_packets,
            sent_errs: self.sent_errs - previous.sent_errs,
            sent_drop: self.sent_drop - previous.sent_drop,
            sent_fifo: self.sent_fifo - previous.sent_fifo,
            sent_colls: self.sent_colls - previous.sent_colls,
            sent_carrier: self.sent_carrier - previous.sent_carrier,
            sent_compressed: self.sent_compressed - previous.sent_compressed,
        }
    }
}

impl WindowedObservation for DeviceStatus {
    fn normalize(&self, win: std::time::Duration) -> Self {
        DeviceStatus {
            name: self.name.clone(),
            recv_bytes: window_norm_u64(self.recv_bytes, win),
            recv_packets: window_norm_u64(self.recv_packets, win),
            recv_errs: window_norm_u64(self.recv_errs, win),
            recv_drop: window_norm_u64(self.recv_drop, win),
            recv_fifo: window_norm_u64(self.recv_fifo, win),
            recv_frame: window_norm_u64(self.recv_frame, win),
            recv_compressed: window_norm_u64(self.recv_compressed, win),
            recv_multicast: window_norm_u64(self.recv_multicast, win),
            sent_bytes: window_norm_u64(self.sent_bytes, win),
            sent_packets: window_norm_u64(self.sent_packets, win),
            sent_errs: window_norm_u64(self.sent_errs, win),
            sent_drop: window_norm_u64(self.sent_drop, win),
            sent_fifo: window_norm_u64(self.sent_fifo, win),
            sent_colls: window_norm_u64(self.sent_colls, win),
            sent_carrier: window_norm_u64(self.sent_carrier, win),
            sent_compressed: window_norm_u64(self.sent_compressed, win),
        }
    }
}

impl ProcFSWrapper<InterfaceDeviceStatus> {
    pub(super) fn network_usage(&self) -> BackendResult<Vec<DeviceStatus>> {
        let data = self.data()?;
        let InterfaceDeviceStatus(cur) = data.current.as_ref().ok_or(BackendError::NotAvailable)?;
        let prev = data.previous.as_ref().map(|InterfaceDeviceStatus(p)| p);
        Ok(cur
            .values()
            .map(|n| {
                if let Some(p) = prev.and_then(|h| h.get(&n.name)) {
                    n.diff(p).normalize(data.window.window_duration())
                } else {
                    n.clone()
                }
            })
            .collect())
    }
}
