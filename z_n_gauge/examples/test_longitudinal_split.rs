#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use crunchy::{crarray::shape::Shape, lattice::fields::UpdateField};
use z_n_gauge::{
    experiment::experiment::{
        calculate_lateral_and_longitudinal_correlator, generate_cosine, generate_roots_of_unity,
        Experiment,
    },
    gauge_fields::{edges::EdgeField, lattice::ZNParameters},
};

fn main() {
    const ZORDER: usize = 7;

    let roots_of_unity = generate_roots_of_unity::<ZORDER>();

    const S: usize = 4;
    let shape = Shape::new([S, S, S, S]);

    let sim_parameters = ZNParameters::<ZORDER> {
        beta: 0.1,
        cosines: generate_cosine(),
        lambda: 0.5,
    };

    let mut experiment = Experiment::new(shape, sim_parameters, 0);

    // let mut max_holonomy = 0;
    // for face in experiment.faces.iter() {
    //     if face.data.holonomy > max_holonomy {
    //         max_holonomy = face.data.holonomy;
    //     }
    // }

    // // println!("Max Holonomy: {:?}", max_holonomy);

    // // print_all_correlators(&experiment, S);

    let node_index = [0, 1, 1, 1, 1];

    EdgeField::<4, ZORDER>::update(node_index, &mut experiment.sim, EdgeField { phase: 1 });

    let res = calculate_lateral_and_longitudinal_correlator(&experiment, 1);

    println!("{:?}", res);

    // for p_index in experiment.faces.shape().iter() {
    //     let p = &experiment.faces[p_index];
    //     if p.data.holonomy != 0 {
    //         println!("Index: {:?}, holonomy: {:?}", p_index, p.data.holonomy);
    //     }
    // }

    // let debug_correlator = debug_calculate_distance_correlator(&experiment, 1);

    // println!("\nd=1 Debug Correlator: {:?}", debug_correlator);

    // let (correlator, correlator_differences) = calculate_distance_correlator(&experiment, 1);
    // println!("Correlator: {:?}", correlator);

    // let average_plaquette = measure_average_plaquette(&experiment).0;
    // println!("Average Plaquette: {:?}\n", average_plaquette);

    // println!("Correlator Differences: {:?}", correlator_differences);

    // println!(
    //     "{:?}",
    //     correlator[0] - average_plaquette[0] * average_plaquette[0]
    // );
}
