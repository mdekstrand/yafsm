use log::*;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;

use crate::backend::BackendResult;
use crate::model::GPUStats;

pub(super) struct GPUs {
    nvidia: Option<Nvml>,
}

impl GPUs {
    pub(super) fn init() -> BackendResult<GPUs> {
        let nvidia = match Nvml::init() {
            Ok(n) => Some(n),
            Err(NvmlError::LibloadingError(e)) => {
                debug!("error loading NVML: {}", e);
                None
            }
            Err(NvmlError::DriverNotLoaded | NvmlError::NoPermission) => {
                debug!("NVML could not be loaded");
                None
            }
            Err(e) => return Err(e.into()),
        };
        Ok(GPUs { nvidia })
    }

    pub(super) fn gpu_count(&self) -> u32 {
        let mut count = 0;
        if let Some(nv) = &self.nvidia {
            count += nv.device_count().unwrap_or_default()
        }

        count
    }

    pub(super) fn gpus(&self) -> BackendResult<Vec<GPUStats>> {
        let mut stats = Vec::new();
        if let Err(e) = self.nvidia_gpus(&mut stats) {
            warn!("error retrieving GPUs: {}", e);
        }

        Ok(stats)
    }

    pub(super) fn nvidia_gpus(&self, stats: &mut Vec<GPUStats>) -> BackendResult<u32> {
        let nv = if let Some(nv) = &self.nvidia {
            nv
        } else {
            return Ok(0);
        };

        let n = nv.device_count()?;
        for i in 0..n {
            let dev = nv.device_by_index(i)?;
            let util = dev.utilization_rates()?;
            let mem = dev.memory_info()?;
            let power = match dev.power_usage() {
                Ok(mw) => Some((mw as f32) * 0.001),
                Err(NvmlError::NotSupported) => None,
                Err(e) => return Err(e.into()),
            };
            let temp = match dev.temperature(TemperatureSensor::Gpu) {
                Ok(t) => Some(t as f32),
                Err(NvmlError::NotSupported) => None,
                Err(e) => return Err(e.into()),
            };
            stats.push(GPUStats {
                name: dev.name()?,
                gpu_util: util.gpu as f32 / 100.0,
                mem_util: mem.used as f32 / mem.total as f32,
                mem_avail: mem.free,
                mem_total: mem.total,
                mem_used: mem.used,
                temp,
                power,
            })
        }

        Ok(n)
    }
}
