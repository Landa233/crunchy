#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use crunchy::crarray::shape::Shape;
use ndarray::{Array, ArrayBase, IxDyn};
use z_n_gauge::{
    experiment::experiment::{
        generate_cosine, record_polyakov_correlators, record_polyakov_loops, Experiment,
    },
    gauge_fields::lattice::ZNParameters,
};

fn main() {
    let sim_par = ZNParameters {
        beta: 0.2,
        cosines: generate_cosine(),
        lambda: 0.0,
    };

    let shape = Shape::new([4, 4, 4, 4]);

    let recordings = 10;
    let l = shape[0];
    let mut experiment: Experiment<7> = Experiment::new(shape.into(), sim_par, 2);

    for _ in 0..100 {
        experiment.sweep();
    }

    let corr_shape = (recordings as usize, l + 1, 2);

    let mut polyakov_loops_correlators_recordings: Array<f32, IxDyn> =
        Array::zeros(corr_shape).into_dyn();

    let mut polyakov_loops_recordings: Array<u8, IxDyn> =
        ArrayBase::zeros((recordings as usize, shape[0], shape[1], shape[2])).into_dyn();

    record_polyakov_loops(&experiment, &mut polyakov_loops_recordings, 0);

    record_polyakov_correlators(
        &experiment,
        &mut polyakov_loops_correlators_recordings,
        &polyakov_loops_recordings,
        0,
    );
}
