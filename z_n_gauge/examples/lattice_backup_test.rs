use std::{fs::File, io};

use ndarray::{Array, IxDyn};
use npyz::{AutoSerialize, WriterBuilder};
use rand_pcg::Pcg64Mcg;
use z_n_gauge::{
    experiment::{backup::LatticeBackup, backup_serializer::backup_dtype},
    sheduler::executor::RunInfo,
};

fn main() {
    // let run_info = RunInfo {
    //     shape: [1, 2, 3, 4],
    //     z_order: 7,
    //     beta: 0.1,
    //     lambda: 1.7,
    //     recordings: 123,
    //     recording_skip: 257,
    //     recordings_until_backup: 1111,
    //     rng_seed: 55,
    // };

    let run_info = RunInfo {
        experiment_parameters: z_n_gauge::sheduler::executor::ExperimentParameters {
            shape: [1, 2, 3, 4],
            z_order: 7,
            beta: 0.1,
            lambda: 1.7,
            rng_seed: 55,
        },
        run_parameters: z_n_gauge::sheduler::executor::RunParameters {
            recordings: 123,
            recording_skip: 257,
            recordings_until_backup: 1111,
        },
    };

    let rng = Pcg64Mcg::new(0);
    let recordings: Array<u8, IxDyn> = Array::from_shape_vec([2, 2], vec![1; 4])
        .unwrap()
        .into_dyn();

    let lattice_backup = LatticeBackup {
        run_info,
        edges_flat_data: vec![0, 1, 2, 3, 4],
        rng_gen: rng,
        performed_updates: 22,
        accepted_updates: 44,
        recordings,
    };

    let mut file = io::BufWriter::new(File::create("zzz.npy").unwrap());
    let mut writer = npyz::WriteOptions::new()
        .dtype(backup_dtype(&lattice_backup))
        // .shape(&[1])
        .writer(&mut file)
        .begin_1d()
        .unwrap();

    writer.push(&lattice_backup).unwrap();
    writer.finish().unwrap();

    let bytes = std::fs::read("zzz.npy").unwrap();
    let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

    let deserialized_backup: Vec<LatticeBackup> = reader.into_vec().unwrap();

    if deserialized_backup[0] != lattice_backup {
        panic!("Deserialized backup is different from the original one");
    } else {
        println!("Deserialized backup is the same as the original one");
    }
}
