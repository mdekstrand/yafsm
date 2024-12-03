//! Summary box displays.

use ratatui::style::{Color, Style, Stylize};

use friendly::scalar;

use crate::backend::BackendError;
use crate::backend::BackendResult;
use crate::model::cpu::ExtendedCPU;
use crate::model::ExtendedMemory;
use crate::model::MonitorData;
use crate::view::util::fmt_si_val;
use crate::view::widgets::infocols::{ICEntry, InfoCols};

pub fn cpu_summary(state: &dyn MonitorData) -> BackendResult<InfoCols> {
    let cpu = state.global_cpu()?;
    let mut display = InfoCols::new().add(
        ICEntry::new("CPU")
            .pct(state.global_cpu()?.utilization * 100.0)
            .value_style(Style::new().bold()),
    );
    if let ExtendedCPU::Linux(lcpu) = cpu.extended {
        display = display
            .add_pct("user", lcpu.user * 100.0)
            .add_pct("system", lcpu.system * 100.0)
            .add_pct("iowait", lcpu.iowait * 100.0)
            .add(
                ICEntry::new("idle")
                    .pct(lcpu.idle * 100.0)
                    .value_style(Style::new().fg(Color::White)),
            )
            .add(
                ICEntry::new("nice")
                    .pct(lcpu.nice * 100.0)
                    .value_style(Style::new().fg(Color::White)),
            )
            .add(
                ICEntry::new("irq")
                    .pct(lcpu.irq * 100.0)
                    .value_style(Style::new().fg(Color::White)),
            )
            .add_pct("steal", lcpu.steal * 100.0)
    }
    Ok(display)
}

pub fn memory_summary(state: &dyn MonitorData) -> BackendResult<InfoCols> {
    let mem = state.memory()?;
    let ic = InfoCols::new()
        .add(
            ICEntry::new("MEMORY")
                .pct(mem.used_frac() * 100.0)
                .value_style(Style::new().bold()),
        )
        .add_bytes("total", mem.total)
        .add_bytes("used", mem.used)
        .add_bytes("avail", mem.free + mem.freeable);
    let ic = match mem.extended {
        ExtendedMemory::None => ic,
        ExtendedMemory::Linux(linux) => {
            let mut ic = ic
                .add_bytes("active", linux.active)
                .add_bytes("inacti", linux.inactive)
                .add_bytes("cached", linux.cached);
            if let Some(zfs) = linux.arc {
                ic = ic.add_bytes("zfsarc", zfs)
            }
            if let Some(shared) = linux.shared {
                ic = ic.add_bytes("shared", shared);
            }
            if let Some(reclaim) = linux.reclaimable {
                ic = ic.add_bytes("reclm", reclaim);
            }
            ic = ic.add_bytes("buffers", linux.buffers);
            ic
        }
    };
    Ok(ic)
}

pub fn swap_summary(state: &dyn MonitorData) -> BackendResult<InfoCols> {
    let swp = state.swap()?;
    let ic = InfoCols::new()
        .add(
            ICEntry::new("SWAP")
                .pct(swp.used_frac() * 100.0)
                .value_style(Style::new().bold()),
        )
        .add_bytes("total", swp.total)
        .add_bytes("used", swp.used)
        .add_bytes("free", swp.free);
    Ok(ic)
}

pub fn gpu_summary(state: &dyn MonitorData) -> BackendResult<InfoCols> {
    let gpus = state.gpus()?;
    if gpus.is_empty() {
        Err(BackendError::NotAvailable)
    } else if gpus.len() == 1 {
        let gpu = &gpus[0];
        let mut ic = InfoCols::new()
            .add(ICEntry::new(gpu.name.clone()))
            .add_pct("gpu", gpu.gpu_util * 100.0)
            .add_pct("mem", gpu.mem_util * 100.0);
        if let Some(pow) = gpu.power {
            ic = ic.add_str("power", format!("{}W", fmt_si_val(pow)))
        }
        Ok(ic)
    } else {
        let n = gpus.len();
        let tot_gpu: f32 = gpus.iter().map(|g| g.gpu_util).sum();
        let tot_mem_util: f32 = gpus.iter().map(|g| g.mem_util).sum();
        let tot_mem_avail: u64 = gpus.iter().map(|g| g.mem_avail).sum();
        let mut ic = InfoCols::new()
            .add_count("GPUs", n as u64)
            .add_pct("gpu", tot_gpu as f32 / n as f32 / 100.0)
            .add_pct("gpu", tot_mem_util as f32 / n as f32 / 100.0)
            .add_bytes("avail", tot_mem_avail);
        for gpu in gpus.iter() {
            ic = ic
                .add(ICEntry::new(gpu.name.clone()))
                .add_pct("gpu", gpu.gpu_util / 100.0)
                .add_pct("mem", gpu.mem_util / 100.0);
            if let Some(pow) = gpu.power {
                ic = ic.add_value("power", pow)
            }
        }
        Ok(ic)
    }
}

pub fn pressure_summary(state: &dyn MonitorData) -> BackendResult<InfoCols> {
    let press = state.pressure()?;
    Ok(InfoCols::new()
        .add(ICEntry::new("PSI").string("10s"))
        .add_pct("cpu", press.cpu_psi.avg10)
        .add_pct("mem", press.mem_psi.avg10)
        .add_pct("io", press.io_psi.avg10))
}

pub fn load_summary(state: &dyn MonitorData) -> BackendResult<InfoCols> {
    let ncpus = state.cpu_count()? as f32;
    let load = state.load_avg()?;
    Ok(InfoCols::new()
        .add(ICEntry::new("LOAD").string(format!("{}core", state.cpu_count()?)))
        .add(
            ICEntry::new("1min")
                .value(load.one)
                .value_style(Style::new().fg(if load.one >= ncpus {
                    Color::Red
                } else if load.one >= ncpus * 0.5 {
                    Color::Magenta
                } else {
                    Color::White
                })),
        )
        .add(
            ICEntry::new("5min")
                .value(load.five)
                .value_style(Style::new().fg(if load.five >= ncpus {
                    Color::Red
                } else if load.five >= ncpus * 0.5 {
                    Color::Magenta
                } else {
                    Color::White
                })),
        )
        .add(
            ICEntry::new("15min")
                .value(load.fifteen)
                .value_style(Style::new().fg(if load.fifteen >= ncpus {
                    Color::Red
                } else if load.fifteen >= ncpus * 0.5 {
                    Color::Magenta
                } else {
                    Color::White
                })),
        ))
}
