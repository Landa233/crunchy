use std::{fs::File, io::Write};

use rand::{RngCore, SeedableRng};
use z_n_gauge::experiment::backup::LatticeBackup;

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
}
