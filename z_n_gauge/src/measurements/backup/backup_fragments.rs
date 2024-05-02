use npyz::{Deserialize, Serialize};
use rs_to_npy::array_wrapper::ArrayWrapper;
use rs_to_npy_macros::DTypeable;

use crate::measurements::rng_gen_wrapper::RngGenWrapper;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DTypeable)]
pub struct RebootSeed {
    pub last_state: LastState,
    pub experiment_parameters: ExperimentParameters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DTypeable)]
pub struct LastState {
    pub edges: ArrayWrapper<u8>,
    pub rng_gen: RngGenWrapper,
    pub performed_updates: u64,
    pub accepted_updates: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, DTypeable)]
pub struct ExperimentParameters {
    pub shape: [u32; 4],
    pub z_order: u32,
    pub beta: f32,
    pub lambda: f32,
    pub rng_seed: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, DTypeable)]
pub struct RunInfo {
    pub recordings: u32,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
    pub backup_number: u32,
    pub recording_time: i64,
    pub run_id: u32,
}
