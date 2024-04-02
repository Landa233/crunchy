use ndarray::{Array, IxDyn};
use serde::{Deserialize, Serialize};

use crate::experiment::{
    backup::{backup_experiment, LatticeBackup},
    experiment::Experiment,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExecutorParameters {
    pub shape: [usize; 4],

    pub z_order: usize,
    pub beta: f32,
    pub lambda: f32,

    pub recordings: usize,
    pub recording_skip: usize,
    pub recordings_until_backup: usize,

    pub rng_seed: u64,
}

pub fn execute(backup: LatticeBackup) {
    let mut experiment: Experiment<3> = backup.reboot_experiment();

    let ExecutorParameters {
        shape,
        z_order,
        beta,
        lambda,
        recordings,
        recording_skip,
        recordings_until_backup,
        rng_seed,
    } = backup.experiment_parameters;

    let [x_dim, y_dim, z_dim, t_dim] = shape;

    let res_shape = [recordings, x_dim, y_dim, z_dim];
    let zeros: Vec<u8> = vec![0; res_shape.iter().product()];

    let mut polyakov_recordings: Array<u8, IxDyn> =
        Array::from_shape_vec(res_shape, zeros).unwrap().into_dyn();

    let mut prev_backup = 0;
    let mut backup_counter = 0;

    for i in 0..recordings {
        for x in 0..x_dim {
            for y in 0..y_dim {
                for z in 0..z_dim {
                    let mut singe_loop = 0;
                    for t in 0..t_dim {
                        let a = experiment.sim.edges[[3, x, y, z, t]];
                        singe_loop += a.data.phase;
                    }
                    singe_loop %= z_order;
                    polyakov_recordings[[i, x, y, z]] = singe_loop as u8;
                }
            }
        }

        backup_counter += 1;
        if backup_counter == recordings_until_backup {
            let backup = backup_experiment(&experiment, backup.experiment_parameters);

            prev_backup = i;
            backup_counter = 0;
        }

        for _ in 0..recording_skip {
            experiment.sweep();
        }
    }
}
