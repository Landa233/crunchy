use byteorder::{LittleEndian, WriteBytesExt};
use ndarray::{Array, IxDyn};
use npyz::{AutoSerialize, DType, Deserialize, Serialize, TypeWrite};
use rand_pcg::Pcg64Mcg;

use crate::experiment;

#[derive(Debug, Clone, PartialEq)]
pub struct BackUp {
    pub reboot_seed: RebootSeed,
    pub backup_data: BackupData,
    pub run_info: RunInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RebootSeed {
    pub last_state: LastState,
    pub experiment_parameters: ExperimentParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LastState {
    pub edges: Vec<u8>,
    pub rng_gen: Pcg64Mcg,
    pub performed_updates: u64,
    pub accepted_updates: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, AutoSerialize, PartialEq)]
pub struct ExperimentParameters {
    pub shape: [u32; 4],
    pub z_order: u32,
    pub beta: f32,
    pub lambda: f32,
    pub rng_seed: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, AutoSerialize, PartialEq)]
pub struct RunInfo {
    pub recordings: u32,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
    pub backup_number: u32,
    pub recording_time: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BackupData {
    pub recorded_data: Array<u8, IxDyn>,
}

pub fn backup_dtype(backup: &BackUp) -> DType {
    DType::Record(vec![
        npyz::Field {
            name: "reboot_seed".to_string(),
            dtype: reboot_seed_dtype(&backup.reboot_seed),
        },
        npyz::Field {
            name: "backup_data".to_string(),
            dtype: array_dtype(&backup.backup_data.recorded_data),
        },
        npyz::Field {
            name: "run_info".to_string(),
            dtype: <RunInfo as AutoSerialize>::default_dtype(),
        },
    ])
}

pub fn reboot_seed_dtype(reboot_seed: &RebootSeed) -> DType {
    let rng_bytes = bincode::serialize(&reboot_seed.last_state.rng_gen).unwrap();

    let last_state_dtype = DType::Record(vec![
        npyz::Field {
            name: "edges".to_string(),
            dtype: DType::Array(
                reboot_seed.last_state.edges.len() as u64,
                Box::new(DType::Plain("<u1".parse().unwrap())),
            ),
        },
        npyz::Field {
            name: "rng_gen".to_string(),
            dtype: DType::Array(
                rng_bytes.len() as u64,
                Box::new(DType::Plain("<u1".parse().unwrap())),
            ),
        },
        npyz::Field {
            name: "performed_updates".to_string(),
            dtype: DType::Plain("<u8".parse().unwrap()),
        },
        npyz::Field {
            name: "accepted_updates".to_string(),
            dtype: DType::Plain("<u8".parse().unwrap()),
        },
    ]);

    let experiment_parameters_dtype = <ExperimentParameters as AutoSerialize>::default_dtype();

    DType::Record(vec![
        npyz::Field {
            name: "last_state".to_string(),
            dtype: last_state_dtype,
        },
        npyz::Field {
            name: "experiment_parameters".to_string(),
            dtype: experiment_parameters_dtype,
        },
    ])
}

fn array_dtype(array: &Array<u8, IxDyn>) -> DType {
    let shape = array.shape();

    let inner = Box::new(DType::Plain("<u1".parse().unwrap()));

    let mut dtype = inner;
    for dim in shape.iter().rev() {
        dtype = Box::new(DType::Array(*dim as u64, dtype))
    }

    return *dtype;
}
