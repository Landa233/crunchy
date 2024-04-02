#![allow(unused_macros, incomplete_features)]
#![feature(generic_const_exprs)]

// use z_n_gauge::{experiment::experiment::Experiment, gauge_fields::lattice::ZNParameters};

fn generate_cosine<const ZORDER: usize>() -> [f32; ZORDER] {
    let pi = std::f32::consts::PI;
    let mut res = [0.0; ZORDER];

    // Generate perfectly symmetric cosine, otherwise this leads to symmetry breaking
    for i in 0..(ZORDER / 2 + 1) {
        res[i] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        if i != 0 {
            res[ZORDER - i] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        }
    }

    res
}

// fn main() {
//     // let rng_gen = rand_pcg::Pcg64Mcg::from_entropy();

//     // let mut rng_copy = rng_gen.clone();

//     // let mut lattice = LatticeBackup {
//     //     shape: [1, 2, 3, 4],
//     //     edges_flat_data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
//     //     rng_gen,
//     // };

//     // println!("{:?}", rng_copy.next_u32());
//     // println!("{:?}", lattice.rng_gen.next_u32());

//     // println!("{:?}", rng_copy.next_u32());
//     // println!("{:?}", lattice.rng_gen.next_u32());

//     // let bincode_data = bincode::serialize(&lattice).unwrap();

//     // let mut bincode_file = File::create("lattice_backup.bincode").unwrap();
//     // bincode_file.write_all(&bincode_data).unwrap();

//     // let bincode_data = std::fs::read("lattice_backup.bincode").unwrap();

//     // let deserialized = bincode::deserialize::<LatticeBackup>(&bincode_data).unwrap();

//     // println!("{:?}", deserialized);

//     let sim_pars = ZNParameters {
//         beta: 0.5,
//         cosines: generate_cosine::<3>(),
//         lambda: 1.0,
//     };

//     let grid_shape = [4, 4, 4, 4];

//     let mut experiment = Experiment::<3>::new(grid_shape.into(), sim_pars);

//     let exp_clone = experiment.clone();
// }

use ndarray::{s, Array, IxDyn};
use z_n_gauge::{
    experiment::{backup::backup_experiment, experiment::Experiment},
    gauge_fields::lattice::ZNParameters,
    sheduler::executor::ExecutorParameters,
};

fn main() {
    let beta = 0.1;
    let lambda = 1.0;

    let sim_pars = ZNParameters {
        beta,
        cosines: generate_cosine(),
        lambda,
    };

    let grid_shape = [4, 4, 4, 4];

    let mut experiment = Experiment::<3>::new(grid_shape.into(), sim_pars);

    for _ in 0..10 {
        experiment.sweep();
    }

    let experiment_parameters = ExecutorParameters {
        shape: grid_shape,
        z_order: 3,
        beta,
        lambda,
        recordings: 10,
        recording_skip: 1,
        recordings_until_backup: 1,
        rng_seed: 1,
    };

    let backup = backup_experiment(&experiment, experiment_parameters);

    let restored_experiment: Experiment<3> = backup.reboot_experiment();

    println!("{:?}", experiment == restored_experiment);

    let res_shape = [10, 10];
    let zeros: Vec<u8> = vec![0; res_shape.iter().product()];

    let mut a: Array<u8, IxDyn> = Array::from_shape_vec(res_shape, zeros).unwrap().into_dyn();

    println!("{:?}", a.slice(s![0..2, 0..1]));

    // let mut exp_clone = experiment.clone();

    // println!("{:?}", experiment == exp_clone);

    // experiment.sweep();
    // exp_clone.sweep();

    // println!("{:?}", experiment == exp_clone);
}
