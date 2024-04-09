// use ndarray::{s, Array, IxDyn};
// use npyz::{AutoSerialize, Deserialize, Serialize};

// use crate::experiment::experiment::Experiment;

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, AutoSerialize, PartialEq)]
// pub struct RunInfo {
//     pub experiment_parameters: ExperimentParameters,
//     pub run_parameters: RunParameters,
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, AutoSerialize, PartialEq)]
// pub struct ExperimentParameters {
//     pub shape: [u32; 4],

//     pub z_order: u32,
//     pub beta: f32,
//     pub lambda: f32,

//     pub rng_seed: u64,
// }

// #[derive(Debug, Copy, Clone, Serialize, Deserialize, AutoSerialize, PartialEq)]
// pub struct RunParameters {
//     pub recordings: u32,
//     pub recording_skip: u32,
//     pub recordings_until_backup: u32,
// }

// pub fn execute(backup: LatticeBackup) {
//     let mut experiment: Experiment<3> = backup.reboot_experiment();

//     let RunInfo {
//         experiment_parameters: ExperimentParameters { shape, z_order, .. },
//         run_parameters:
//             RunParameters {
//                 recordings,
//                 recording_skip,
//                 recordings_until_backup,
//             },
//     } = backup.run_info;

//     let [x_dim, y_dim, z_dim, t_dim] = shape;

//     let x_dim = x_dim as usize;
//     let y_dim = y_dim as usize;
//     let z_dim = z_dim as usize;
//     let t_dim = t_dim as usize;
//     let recordings = recordings as usize;

//     let res_shape = [recordings, x_dim, y_dim, z_dim];

//     let zeros: Vec<u8> = vec![0; res_shape.iter().product()];

//     let mut polyakov_recordings: Array<u8, IxDyn> =
//         Array::from_shape_vec(res_shape, zeros).unwrap().into_dyn();

//     let mut prev_backup = 0;
//     let mut backup_counter = 0;

//     for i in 0..recordings {
//         for x in 0..x_dim {
//             for y in 0..y_dim {
//                 for z in 0..z_dim {
//                     let mut singe_loop = 0;
//                     for t in 0..t_dim {
//                         let a = experiment.sim.edges[[3, x, y, z, t]];
//                         singe_loop += a.data.phase;
//                     }
//                     singe_loop %= z_order as usize;
//                     polyakov_recordings[[i, x, y, z]] = singe_loop as u8;
//                 }
//             }
//         }

//         backup_counter += 1;
//         if backup_counter == recordings_until_backup {
//             let recording_snipped = polyakov_recordings
//                 .slice(s![prev_backup..i, .., .., ..])
//                 .to_owned()
//                 .into_dyn();
//             let backup = backup_experiment(&experiment, backup.run_info, recording_snipped);

//             prev_backup = i;
//             backup_counter = 0;
//         }

//         for _ in 0..recording_skip {
//             experiment.sweep();
//         }
//     }

// }

use ndarray::{Array, IxDyn};

use crate::experiment::{
    backup::backup::{RebootSeed, RunInfo},
    experiment::Experiment,
};

#[derive(Clone, Copy)]
pub struct ExecutorParameters {
    pub recordings: u32,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
}

pub fn execute<const ZORDER: usize>(exec_par: ExecutorParameters, reboot_seed: RebootSeed) {
    assert!(ZORDER == reboot_seed.experiment_parameters.z_order as usize);

    let mut experiment = Experiment::<ZORDER>::reboot_experiment(reboot_seed);

    let mut backup_number = 0;
    let mut recordings_to_go = exec_par.recordings;
    let recordings_until_backup = exec_par.recordings_until_backup;

    let shape = experiment.sim.shape;

    while recordings_to_go > 0 {
        let recordings = if recordings_to_go > recordings_until_backup {
            recordings_until_backup
        } else {
            recordings_to_go
        };

        let rec_shape = [recordings as usize, shape[0], shape[1], shape[2]];
        let zeros: Vec<u8> = vec![0; rec_shape.iter().product()];

        let mut polyakov_recordings: Array<u8, IxDyn> =
            Array::from_shape_vec(rec_shape, zeros).unwrap().into_dyn();

        for i in 0..recordings {
            experiment.record_experiment(&mut polyakov_recordings, i as usize);
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
        };

        let backup = experiment.backup(polyakov_recordings, run_info);

        todo!("Save backup");

        recordings_to_go -= recordings;
        backup_number += 1;
    }
}
