use std::fs;

use chrono::{Duration, Utc};
use ndarray::{Array, Axis, IxDyn};
use rs_to_npy::array_wrapper::ArrayWrapper;
use rs_to_npy_macros::DTypeable;

use crate::{
    experiment::experiment::Experiment,
    measurements::backup::{
        backup_fragments::{RebootSeed, RunInfo},
        backup_trait::{BackUp, ExecutorParameters, HaltingCondition},
    },
};

#[derive(Debug, Clone, PartialEq, npyz::Serialize, npyz::Deserialize, DTypeable)]
pub struct PolyakovBackup {
    pub reboot_seed: RebootSeed,
    pub polyakov_loops: BackupData,
    pub run_info: RunInfo,
}

#[derive(Debug, Clone, npyz::Serialize, npyz::Deserialize, PartialEq, DTypeable)]
pub struct BackupData {
    pub polyakov_loops: ArrayWrapper<u8>,
}

impl BackUp for PolyakovBackup {
    type BackupData<'a, const ZORDER: usize> =
        (&'a Experiment<{ ZORDER }>, ArrayWrapper<u8>, RunInfo);

    fn new<'a, const ZORDER: usize>(backup_data: Self::BackupData<'a, ZORDER>) -> Self {
        let reboot_seed = backup_data.0.to_seed();

        let polyakov_loops = BackupData {
            polyakov_loops: backup_data.1,
        };

        Self {
            reboot_seed,
            polyakov_loops,
            run_info: backup_data.2,
        }
    }

    fn merge(mut backups: Vec<Self>) -> Self {
        let mut total_recordings = 0;
        let mut array_views = vec![];
        for backup in backups.iter() {
            total_recordings += backup.run_info.recordings;
            array_views.push(backup.polyakov_loops.polyakov_loops.data.view());
        }

        let merged_data = ndarray::concatenate(Axis(0), &array_views).unwrap();

        let mut backup = backups.pop().unwrap();

        backup.run_info.recordings = total_recordings;
        backup.polyakov_loops.polyakov_loops = ArrayWrapper { data: merged_data };
        backup.run_info.backup_number = 1;

        backup
    }

    fn execute<const ZORDER: usize>(exec_par: ExecutorParameters, reboot_seed: RebootSeed) {
        assert!(ZORDER == reboot_seed.experiment_parameters.z_order as usize);
        println!("{:?}", exec_par.run_id);

        fs::create_dir_all(&exec_par.parent_path).unwrap();

        let mut experiment = Experiment::<ZORDER>::reboot_experiment(reboot_seed);
        let recordings_until_backup = exec_par.recordings_until_backup;
        let shape = experiment.sim.shape;

        let (max_recordings, duration) = match exec_par.halting_condition {
            HaltingCondition::Recordings(rec) => (rec, Duration::weeks(20000000)),
            HaltingCondition::Time(duration) => (2_u32.pow(31), *duration),
        };
        let start_time = Utc::now();

        let mut backup_number = 1;
        let mut recordings_counter = 0;

        while recordings_counter < max_recordings && Utc::now() - start_time < duration {
            let recordings = if max_recordings - recordings_counter >= recordings_until_backup {
                recordings_until_backup
            } else {
                max_recordings - recordings_counter
            };

            let rec_shape = [recordings as usize, shape[0], shape[1], shape[2]];
            let zeros: Vec<u8> = vec![0; rec_shape.iter().product()];

            let mut polyakov_recordings: Array<u8, IxDyn> =
                Array::from_shape_vec(rec_shape, zeros).unwrap().into_dyn();

            for i in 0..recordings {
                record_experiment(&experiment, &mut polyakov_recordings, i as usize);
                for _ in 0..exec_par.recording_skip {
                    experiment.sweep();
                }
            }

            let now = chrono::Utc::now();
            let recording_time = now.timestamp();

            let run_info = RunInfo {
                recordings,
                recording_skip: exec_par.recording_skip,
                recordings_until_backup,
                backup_number,
                recording_time,
                run_id: exec_par.run_id,
            };

            let backup = PolyakovBackup::new((
                &experiment,
                ArrayWrapper {
                    data: polyakov_recordings,
                },
                run_info,
            ));

            let backup_folder_name = format!("{}/{}.npy", exec_par.parent_path, backup_number);

            backup.write_to_file(backup_folder_name);

            recordings_counter += recordings;
            backup_number += 1;
        }
    }
}

pub fn record_experiment<const ZORDER: usize>(
    experiment: &Experiment<ZORDER>,
    rec_array: &mut Array<u8, IxDyn>,
    rec_index: usize,
) {
    let [x_dim, y_dim, z_dim, t_dim] = *experiment.shape;

    for x in 0..x_dim {
        for y in 0..y_dim {
            for z in 0..z_dim {
                let mut singe_loop = 0;
                for t in 0..t_dim {
                    let a = experiment.sim.edges[[3, x, y, z, t]];
                    singe_loop += a.data.phase;
                }
                singe_loop %= ZORDER as usize;
                rec_array[[rec_index, x, y, z]] = singe_loop as u8;
            }
        }
    }
}
