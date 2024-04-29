use std::path::Path;

use ndarray::{Array, Axis, IxDyn};
use npyz::{AutoSerialize, DType, Deserialize, Serialize};
use rand_pcg::Pcg64Mcg;

#[derive(Debug, Clone, PartialEq)]
pub struct BackUp {
    pub reboot_seed: RebootSeed,
    pub backup_data: BackupData,
    pub run_info: RunInfo,
}

impl BackUp {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let bytes = std::fs::read(path).unwrap();
        let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

        let mut deserialized_backup: Vec<BackUp> = reader.into_vec().unwrap();
        deserialized_backup.remove(0)
    }

    pub fn merge_backups(mut backups: Vec<BackUp>) -> BackUp {
        let mut total_recordings = 0;
        let mut array_views = vec![];
        for backup in backups.iter() {
            total_recordings += backup.run_info.recordings;
            array_views.push(backup.backup_data.recorded_data.view());
        }

        let merged_data = ndarray::concatenate(Axis(0), &array_views).unwrap();

        let mut backup = backups.pop().unwrap();

        backup.run_info.recordings = total_recordings;
        backup.backup_data.recorded_data = merged_data.into_dyn();
        backup.run_info.backup_number = 1;

        backup
    }
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, AutoSerialize, PartialEq, Default)]
pub struct RunInfo {
    pub recordings: u32,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
    pub backup_number: u32,
    pub recording_time: i64,
    pub run_id: u32,
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
