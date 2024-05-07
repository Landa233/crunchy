use std::fs;

use crate::experiment::experiment::{
    record_correlators, record_polyakov_correlators, record_polyakov_loops,
};
use crate::measurements::backup::backup_fragments::{RebootSeed, RunInfo};
use crate::measurements::backup::backup_trait::ExecutorParameters;
use crate::{
    experiment::experiment::Experiment,
    measurements::backup::backup_trait::{BackUp, HaltingCondition},
};
use chrono::{Duration, Utc};
use ndarray::{Array, ArrayBase, IxDyn};
use rs_to_npy::array_wrapper::ArrayWrapper;
use rs_to_npy_macros::DTypeable;

#[derive(Debug, Clone, PartialEq, npyz::Serialize, npyz::Deserialize, DTypeable)]
pub struct SavePloopPloopCorrCorr {
    pub reboot_seed: RebootSeed,
    pub run_info: RunInfo,
    pub ploop_corr_recordings: PloopCorrRecordings,
}

#[derive(Debug, Clone, PartialEq, npyz::Serialize, npyz::Deserialize, DTypeable)]
pub struct PloopCorrRecordings {
    pub polyakov_loops: ArrayWrapper<u8>,
    pub polyakov_loop_correlators: ArrayWrapper<[f32; 2]>,
    pub pp_correlators: ArrayWrapper<[f32; 2]>,
}

impl BackUp for SavePloopPloopCorrCorr {
    type BackupData<'a, const ZORDER: usize> =
        (&'a Experiment<{ ZORDER }>, PloopCorrRecordings, RunInfo);

    fn new<'a, const ZORDER: usize>(backup_data: Self::BackupData<'a, ZORDER>) -> Self {
        let reboot_seed = backup_data.0.to_seed();

        let ploop_corr_recordings = backup_data.1;

        Self {
            reboot_seed,
            run_info: backup_data.2,
            ploop_corr_recordings,
        }
    }

    fn merge(_backups: Vec<Self>) -> Self {
        todo!()
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

            let l = shape[0];

            let mut polyakov_loops_recordings: Array<u8, IxDyn> =
                ArrayBase::zeros((recordings as usize, shape[0], shape[1], shape[2])).into_dyn();

            let corr_shape = (recordings as usize, l + 1);
            let zeros = vec![[0.0; 2]; corr_shape.0 * corr_shape.1];
            let mut polyakov_loops_correlators_recordings: Array<[f32; 2], IxDyn> =
                Array::from_shape_vec(corr_shape, zeros.clone())
                    .unwrap()
                    .into_dyn();

            let mut pp_correlators_recordings: Array<[f32; 2], IxDyn> =
                Array::from_shape_vec(corr_shape, zeros).unwrap().into_dyn();

            for i in 0..recordings {
                record_correlators(&experiment, &mut pp_correlators_recordings, i as usize);

                // The order of the following function matters
                record_polyakov_loops(&experiment, &mut polyakov_loops_recordings, i as usize);
                record_polyakov_correlators(
                    &experiment,
                    &mut polyakov_loops_correlators_recordings,
                    &polyakov_loops_recordings,
                    i as usize,
                );

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

            let backup = Self::new((
                &experiment,
                PloopCorrRecordings {
                    polyakov_loops: ArrayWrapper {
                        data: polyakov_loops_recordings.into_dyn(),
                    },
                    polyakov_loop_correlators: ArrayWrapper {
                        data: polyakov_loops_correlators_recordings,
                    },
                    pp_correlators: ArrayWrapper {
                        data: pp_correlators_recordings,
                    },
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
