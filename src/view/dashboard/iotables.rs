//! Tables for IO and other meters on the left side.

use anyhow::Result;
use itertools::Itertools;

use crate::{
    model::MonitorData,
    view::{util::fmt_int_bytes, widgets::tablegrp::TableGroup},
};

pub fn render_network(state: &dyn MonitorData, tg: &mut TableGroup) -> Result<()> {
    let nets = state
        .networks()?
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
    Ok(())
}

pub fn render_filesystems(state: &dyn MonitorData, tg: &mut TableGroup) -> Result<()> {
    let disks = state
        .filesystems()?
        .into_iter()
        .sorted_by(|n1, n2| n1.mount_point.cmp(&n2.mount_point))
        .collect_vec();
    let tbl = tg.add_table("FILESYSTEMS", ["Used", "Total"]);
    for fs in disks {
        tbl.add_row(
            fs.mount_point,
            [fmt_int_bytes(fs.used), fmt_int_bytes(fs.total)],
        )
    }
    Ok(())
}
