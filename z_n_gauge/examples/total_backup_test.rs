use std::{fs::File, io};

use ndarray::{Array, IxDyn};
use npyz::WriterBuilder;
use rand_pcg::Pcg64Mcg;
use z_n_gauge::experiment::backup::backup::{
    backup_dtype, BackUp, BackupData, ExperimentParameters, LastState, RebootSeed, RunInfo,
};

fn main() {
    let rng = Pcg64Mcg::new(0);

    let last_state = LastState {
        edges: vec![1, 2, 3, 4, 5],
        rng_gen: rng,
        performed_updates: 22,
        accepted_updates: 44,
    };

    let experiment_parameters = ExperimentParameters {
        shape: [1, 2, 3, 4],
        z_order: 7,
        beta: 0.1,
        lambda: 1.7,
        rng_seed: 55,
    };

    let reboot_seed = RebootSeed {
        last_state,
        experiment_parameters,
    };

    let recorded_data: Array<u8, IxDyn> = Array::from_shape_vec([2, 2], vec![1; 4])
        .unwrap()
        .into_dyn();

    let backup_data = BackupData { recorded_data };

    let run_info = RunInfo {
        recordings: 123,
        recording_skip: 257,
        recordings_until_backup: 1111,
        backup_number: 7,
        recording_time: 102,
    };

    let backup = BackUp {
        reboot_seed,
        backup_data,
        run_info,
    };

    let backup_copy = backup.clone();

    let mut file = io::BufWriter::new(File::create("zzz.npy").unwrap());
    let mut writer = npyz::WriteOptions::new()
        .dtype(backup_dtype(&backup))
        // .shape(&[1])
        .writer(&mut file)
        .begin_1d()
        .unwrap();

    writer.push(&backup).unwrap();
    writer.finish().unwrap();

    let bytes = std::fs::read("zzz.npy").unwrap();
    let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

    let deserialized_backup: Vec<BackUp> = reader.into_vec().unwrap();

    println!("{:?}", deserialized_backup[0]);

    assert_eq!(backup_copy, deserialized_backup[0]);
    println!("{:?}", "Round trip successful!");
}
