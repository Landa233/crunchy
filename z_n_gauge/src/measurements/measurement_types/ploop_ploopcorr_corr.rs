use std::fs;

use crate::experiment::experiment::{
    record_average_plaquette, record_correlators, record_lat_and_long_corr,
    record_polyakov_correlators, record_polyakov_loops,
};
use crate::measurements::backup::backup_fragments::{RebootSeed, RunInfo};
use crate::measurements::backup::backup_trait::ExecutorParameters;
use crate::measurements::settings::HaltingCondition;
use crate::{experiment::experiment::Experiment, measurements::backup::backup_trait::BackUp};
use chrono::{Duration, Utc};
use ndarray::{Array, ArrayBase, Axis, IxDyn};
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
    pub polyakov_loop_correlators: ArrayWrapper<f32>,
    pub pp_correlators: ArrayWrapper<f32>,
    pub average_plaquette: ArrayWrapper<f32>,
    pub pp_correlators_differences: ArrayWrapper<u64>,
    pub average_plaquette_integer: ArrayWrapper<u64>,
    pub lateral_correlators: ArrayWrapper<u64>,
    pub longitudinal_correlators: ArrayWrapper<u64>,
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

    fn merge(mut backups: Vec<Self>) -> Self {
        let mut total_recordings = 0;
        let mut polyakov_loops_views = vec![];
        let mut polyakov_loop_correlators_views = vec![];
        let mut pp_correlators_views = vec![];
        let mut average_plaquette_views = vec![];
        let mut pp_correlators_differences_views = vec![];
        let mut average_plaquette_integer_views = vec![];
        let mut lateral_correlators_views = vec![];
        let mut longitudinal_correlators_views = vec![];

        for backup in backups.iter() {
            total_recordings += backup.run_info.recordings;
            // array_views.push(backup.polyakov_loops.polyakov_loops.data.view());
            polyakov_loops_views.push(backup.ploop_corr_recordings.polyakov_loops.data.view());
            polyakov_loop_correlators_views.push(
                backup
                    .ploop_corr_recordings
                    .polyakov_loop_correlators
                    .data
                    .view(),
            );
            pp_correlators_views.push(backup.ploop_corr_recordings.pp_correlators.data.view());
            average_plaquette_views
                .push(backup.ploop_corr_recordings.average_plaquette.data.view());
            pp_correlators_differences_views.push(
                backup
                    .ploop_corr_recordings
                    .pp_correlators_differences
                    .data
                    .view(),
            );
            average_plaquette_integer_views.push(
                backup
                    .ploop_corr_recordings
                    .average_plaquette_integer
                    .data
                    .view(),
            );
            lateral_correlators_views
                .push(backup.ploop_corr_recordings.lateral_correlators.data.view());
            longitudinal_correlators_views.push(
                backup
                    .ploop_corr_recordings
                    .longitudinal_correlators
                    .data
                    .view(),
            );
        }

        // let merged_data = ndarray::concatenate(Axis(0), &array_views).unwrap();
        let merged_polyakov_loops = ndarray::concatenate(Axis(0), &polyakov_loops_views).unwrap();
        let merged_polyakov_loop_correlators =
            ndarray::concatenate(Axis(0), &polyakov_loop_correlators_views).unwrap();
        let merged_pp_correlators = ndarray::concatenate(Axis(0), &pp_correlators_views).unwrap();
        let merged_average_plaquette =
            ndarray::concatenate(Axis(0), &average_plaquette_views).unwrap();
        let merged_pp_correlators_differences =
            ndarray::concatenate(Axis(0), &pp_correlators_differences_views).unwrap();
        let merged_average_plaquette_integer =
            ndarray::concatenate(Axis(0), &average_plaquette_integer_views).unwrap();

        let merged_lateral_correlators =
            ndarray::concatenate(Axis(0), &lateral_correlators_views).unwrap();
        let merged_longitudinal_correlators =
            ndarray::concatenate(Axis(0), &longitudinal_correlators_views).unwrap();

        let mut backup = backups.pop().unwrap();

        backup.run_info.recordings = total_recordings;
        backup.ploop_corr_recordings = PloopCorrRecordings {
            polyakov_loops: ArrayWrapper {
                data: merged_polyakov_loops,
            },
            polyakov_loop_correlators: ArrayWrapper {
                data: merged_polyakov_loop_correlators,
            },
            pp_correlators: ArrayWrapper {
                data: merged_pp_correlators,
            },
            average_plaquette: ArrayWrapper {
                data: merged_average_plaquette,
            },
            pp_correlators_differences: ArrayWrapper {
                data: merged_pp_correlators_differences,
            },
            average_plaquette_integer: ArrayWrapper {
                data: merged_average_plaquette_integer,
            },
            lateral_correlators: ArrayWrapper {
                data: merged_lateral_correlators,
            },
            longitudinal_correlators: ArrayWrapper {
                data: merged_longitudinal_correlators,
            },
        };
        backup.run_info.backup_number = 1;

        backup
    }

    fn execute<const ZORDER: usize>(exec_par: ExecutorParameters, reboot_seed: RebootSeed) {
        if ZORDER != reboot_seed.experiment_parameters.z_order as usize {
            panic!(
                "ZORDER mismatch: relaunch_zorder = {}, and reboot_seed.z_order = {}",
                ZORDER, reboot_seed.experiment_parameters.z_order
            );
        }
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

        let mut backup_number = exec_par.backup_number;
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

            let corr_shape = (recordings as usize, l + 1, 2);
            let mut polyakov_loops_correlators_recordings: Array<f32, IxDyn> =
                Array::zeros(corr_shape).into_dyn();

            let mut pp_correlators_recordings: Array<f32, IxDyn> =
                Array::zeros(corr_shape).into_dyn();

            let mut average_plaquette_recordings: Array<f32, IxDyn> =
                Array::zeros((recordings as usize, 2)).into_dyn();

            let corr_differences_shape = (recordings as usize, l + 1, ZORDER);
            let mut pp_correlators_differences: Array<u64, IxDyn> =
                Array::zeros(corr_differences_shape).into_dyn();

            let average_integer_plaquette_shape = (recordings as usize, ZORDER);
            let mut average_plaquette_integer: Array<u64, IxDyn> =
                Array::zeros(average_integer_plaquette_shape).into_dyn();

            let mut lateral_correlators: Array<u64, IxDyn> =
                Array::zeros(corr_differences_shape).into_dyn();
            let mut longitudinal_correlators: Array<u64, IxDyn> =
                Array::zeros(corr_differences_shape).into_dyn();

            for i in 0..recordings {
                record_correlators(
                    &experiment,
                    &mut pp_correlators_recordings,
                    &mut pp_correlators_differences,
                    i as usize,
                );

                record_average_plaquette(
                    &experiment,
                    &mut average_plaquette_recordings,
                    &mut average_plaquette_integer,
                    i as usize,
                );

                record_lat_and_long_corr(
                    &experiment,
                    &mut longitudinal_correlators,
                    &mut lateral_correlators,
                    i as usize,
                );

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
                    average_plaquette: ArrayWrapper {
                        data: average_plaquette_recordings,
                    },
                    pp_correlators_differences: ArrayWrapper {
                        data: pp_correlators_differences,
                    },
                    average_plaquette_integer: ArrayWrapper {
                        data: average_plaquette_integer,
                    },
                    lateral_correlators: ArrayWrapper {
                        data: lateral_correlators,
                    },
                    longitudinal_correlators: ArrayWrapper {
                        data: longitudinal_correlators,
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

    fn reboot_seed(&self) -> RebootSeed {
        self.reboot_seed.clone()
    }

    fn run_info(&self) -> RunInfo {
        self.run_info
    }
}
