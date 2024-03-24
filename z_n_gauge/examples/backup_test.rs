use std::{fs::File, io::Write};

use crunchy::lattice::simulation::simulation::Simulation;
use rand::{RngCore, SeedableRng};
use z_n_gauge::{
    experiment::{backup::LatticeBackup, experiment::Experiment},
    gauge_fields::lattice::ZNParameters,
};

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

fn main() {
    // let rng_gen = rand_pcg::Pcg64Mcg::from_entropy();

    // let mut rng_copy = rng_gen.clone();

    // let mut lattice = LatticeBackup {
    //     shape: [1, 2, 3, 4],
    //     edges_flat_data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    //     rng_gen,
    // };

    // println!("{:?}", rng_copy.next_u32());
    // println!("{:?}", lattice.rng_gen.next_u32());

    // println!("{:?}", rng_copy.next_u32());
    // println!("{:?}", lattice.rng_gen.next_u32());

    // let bincode_data = bincode::serialize(&lattice).unwrap();

    // let mut bincode_file = File::create("lattice_backup.bincode").unwrap();
    // bincode_file.write_all(&bincode_data).unwrap();

    // let bincode_data = std::fs::read("lattice_backup.bincode").unwrap();

    // let deserialized = bincode::deserialize::<LatticeBackup>(&bincode_data).unwrap();

    // println!("{:?}", deserialized);

    let sim_pars = ZNParameters {
        beta: 0.5,
        cosines: generate_cosine::<3>(),
        lambda: 1.0,
    };

    let grid_shape = [4, 4, 4, 4];

    let mut experiment = Experiment::<3>::new(grid_shape.into(), sim_pars);
}
