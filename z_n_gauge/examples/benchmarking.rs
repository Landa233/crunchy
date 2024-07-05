#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use crunchy::crarray::shape::Shape;
use z_n_gauge::{
    experiment::experiment::{generate_cosine, Experiment},
    gauge_fields::lattice::ZNParameters,
};

fn main() {
    let shape = Shape::new([16, 16, 16, 16]);
    let sim_par = ZNParameters {
        beta: 3.0,
        cosines: generate_cosine(),
        lambda: 0.0,
    };

    let mut experiment = Experiment::<7>::new(shape, sim_par, 0);

    // time 10000 sweeps
    let sweeps = 100;
    let start = std::time::Instant::now();
    for _ in 0..sweeps {
        experiment.sweep();
    }
    let end = std::time::Instant::now();
    println!("Time for {} sweeps: {:?}", sweeps, end - start);
    println!("Time for 1 sweep: {:?}", (end - start) / sweeps);
}
