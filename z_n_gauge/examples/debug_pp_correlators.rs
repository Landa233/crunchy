#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::collections::HashMap;

use crunchy::{
    crarray::{ind::Ind, shape::Shape},
    lattice::fields::UpdateField,
};
use z_n_gauge::{
    experiment::{
        self,
        experiment::{
            calculate_distance_correlator, generate_cosine, measure_average_plaquette, Experiment,
        },
    },
    gauge_fields::{edges::EdgeField, lattice::ZNParameters},
};

pub fn generate_roots_of_unity<const ZORDER: usize>() -> [[f32; 2]; ZORDER] {
    let pi = std::f32::consts::PI;
    let mut res = [[0.0; 2]; ZORDER];

    // Generate perfectly symmetric cosine, otherwise this leads to symmetry breaking
    for i in 0..(ZORDER / 2 + 1) {
        res[i][0] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        res[i][1] = (i as f32 * 2.0 * pi / ZORDER as f32).sin();

        if i != 0 {
            res[ZORDER - i][0] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
            res[ZORDER - i][1] = -(i as f32 * 2.0 * pi / ZORDER as f32).sin();
        }
    }

    res
}

fn print_all_correlators<const ZORDER: usize>(experiment: &Experiment<ZORDER>, S: usize) {
    for d in 0..S {
        let corr = debug_calculate_distance_correlator(&experiment, d);
        println!("d:{d}, corr:{:?}", corr);

        let average_plaquette = measure_average_plaquette(&experiment);
        println!("Average Plaquette: {:?}\n", average_plaquette);
    }
}

pub fn debug_calculate_distance_correlator<const ZORDER: usize>(
    experiment: &Experiment<ZORDER>,
    d: usize,
) -> [f32; 2] {
    let roots_of_unity = generate_roots_of_unity::<ZORDER>();

    let mut counter = 0;
    let mut total_correlator = [0.0, 0.0];

    let mut ds = vec![];
    for i in 0..4 {
        let mut d_vec = [0; 4];
        d_vec[i] = d;
        ds.push(Ind::<4>::new(d_vec));
    }

    let faces_shape = experiment.sim.faces.shape();

    // create a hashmap to store correlator counters that are indexed by [u32;2]
    let mut correlator_counters: HashMap<[u32; 2], u32> = HashMap::new();

    let mut all_counter = 0;

    for d in ds {
        for plaquette_plan_index in 0..faces_shape[0] {
            for x in 0..faces_shape[1] {
                for y in 0..faces_shape[2] {
                    for z in 0..faces_shape[3] {
                        for t in 0..faces_shape[4] {
                            let x = Ind::<4>::new([x, y, z, t]);
                            let x_d =
                                ((x + d) % *experiment.sim.shape).prepend(plaquette_plan_index);
                            let x: Ind<5> = x.prepend(plaquette_plan_index);

                            let faces = &experiment.sim.faces;
                            let pp_correlator = (ZORDER + faces[*x].data.holonomy
                                - faces[*x_d].data.holonomy)
                                % ZORDER;

                            all_counter += 1;

                            if pp_correlator != 0 {
                                println!(
                                    " x: {:?}, x_d: {:?}, pp_correlator: {:?}",
                                    x, x_d, pp_correlator
                                );

                                total_correlator[0] += roots_of_unity[pp_correlator][0];
                                total_correlator[1] += roots_of_unity[pp_correlator][1];

                                counter += 1;

                                // check if [faces[*x].data.holonomy, faces[*x_d].data.holonomy] is a key of the hashmap
                                let key = [
                                    faces[*x].data.holonomy as u32,
                                    faces[*x_d].data.holonomy as u32,
                                ];
                                let count = correlator_counters.entry(key).or_insert(0);
                                *count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // print the hashmap
    for (key, value) in correlator_counters.iter() {
        println!("key: {:?}, value: {:?}", key, value);
    }

    println!("Number of Nonzero correlators {:?}", counter);
    println!("Number of correlators {:?}", all_counter);

    total_correlator[0] /= counter as f32;
    total_correlator[1] /= counter as f32;

    total_correlator
}

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

    let mut max_holonomy = 0;
    for face in experiment.faces.iter() {
        if face.data.holonomy > max_holonomy {
            max_holonomy = face.data.holonomy;
        }
    }

    // println!("Max Holonomy: {:?}", max_holonomy);

    // print_all_correlators(&experiment, S);

    let node_index = [0, 1, 1, 1, 1];

    EdgeField::<4, ZORDER>::update(node_index, &mut experiment.sim, EdgeField { phase: 1 });

    for p_index in experiment.faces.shape().iter() {
        let p = &experiment.faces[p_index];
        if p.data.holonomy != 0 {
            println!("Index: {:?}, holonomy: {:?}", p_index, p.data.holonomy);
        }
    }

    let debug_correlator = debug_calculate_distance_correlator(&experiment, 1);

    println!("\nd=1 Debug Correlator: {:?}", debug_correlator);

    let (correlator, correlator_differences) = calculate_distance_correlator(&experiment, 1);
    println!("Correlator: {:?}", correlator);

    let average_plaquette = measure_average_plaquette(&experiment);
    println!("Average Plaquette: {:?}\n", average_plaquette);

    println!("Correlator Differences: {:?}", correlator_differences);

    println!(
        "{:?}",
        correlator[0] - average_plaquette[0] * average_plaquette[0]
    );
}
