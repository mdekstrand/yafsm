//! Tables for IO and other meters on the left side.

use anyhow::Result;
use itertools::Itertools;
use ratatui::prelude::*;

use crate::{
    backend::error::BackendErrorFilter,
    model::MonitorData,
    view::{
        util::{fmt_bytes, fmt_int_bytes},
        widgets::tablegrp::TableGroup,
    },
};

pub fn render_network(state: &dyn MonitorData, tg: &mut TableGroup) -> Result<()> {
    if let Some(nets) = state.networks().acceptable_to_opt()? {
        let nets = nets
            .into_iter()
            .filter(|n| n.name != "lo0")
            .sorted_by(|n1, n2| n1.name.cmp(&n2.name))
            .collect_vec();
        let tbl = tg.add_table("NETWORK", ["RB/s", "WB/s"]);
        for n in nets {
            tbl.add_row(
                n.name,
                [fmt_int_bytes(n.rx_bytes), fmt_int_bytes(n.tx_bytes)],
            )
        }
    }
    Ok(())
}

pub fn render_disks(state: &dyn MonitorData, tg: &mut TableGroup) -> Result<()> {
    if let Some(disks) = state.disk_io().acceptable_to_opt()? {
        let disks = disks
            .into_iter()
            .sorted_by(|n1, n2| n1.name.cmp(&n2.name))
            .collect_vec();
        let tbl = tg.add_table("DISK", ["RB/s", "WB/s"]);
        for d in disks {
            tbl.add_row(
                d.name,
                [fmt_int_bytes(d.rx_bytes), fmt_int_bytes(d.tx_bytes)],
            )
        }
    }
    Ok(())
}

pub fn render_filesystems(state: &dyn MonitorData, tg: &mut TableGroup) -> Result<()> {
    if let Some(disks) = state.filesystems().acceptable_to_opt()? {
        let disks = disks
            .into_iter()
            .sorted_by(|n1, n2| n1.mount_point.cmp(&n2.mount_point))
            .collect_vec();
        let tbl = tg.add_table("FILESYSTEMS", ["Used", "Total"]);
        for fs in disks {
            let frac = fs.utilization();
            let used = Span::from(fmt_bytes(fs.used));
            let tot = Span::from(fmt_bytes(fs.total));
            let used = if frac >= 0.9 {
                used.bold().fg(Color::Red)
            } else if frac >= 0.8 {
                used.bold().fg(Color::Yellow)
            } else if frac >= 0.7 {
                used.bold().fg(Color::Magenta)
            } else if frac >= 0.5 {
                used.bold().fg(Color::Blue)
            } else {
                used.fg(Color::Green)
            };
            tbl.add_row(fs.mount_point, [used, tot])
        }
    }
    Ok(())
}
