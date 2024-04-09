#![feature(generic_const_exprs)]

use crunchy::crarray::shape::Shape;
use ndarray::Array;
use z_n_gauge::{
    experiment::{
        backup::backup::{LastState, RebootSeed, RunInfo},
        experiment::{generate_cosine, Experiment},
    },
    gauge_fields::lattice::ZNParameters,
    sheduler::executor::{execute, ExecutorParameters},
};

fn main() {
    let shape = Shape::<4>::new([4, 4, 4, 4]);
    let z_n_parameters = ZNParameters::<3> {
        beta: 0.1,
        cosines: generate_cosine(),
        lambda: 1.0,
    };

    let experiment = Experiment::new(shape, z_n_parameters, 2);

    let run_info = RunInfo::default();
    let rec = Array::from_shape_vec([2, 2], vec![1; 4])
        .unwrap()
        .into_dyn();

    let backup = experiment.backup(rec, run_info);

    let executor_parameters = ExecutorParameters {
        recordings: 1001,
        recording_skip: 1,
        recordings_until_backup: 1000,
        run_id: 1,
        parent_path: "_test".to_string(),
    };

    execute::<3>(executor_parameters, backup.reboot_seed);
}
