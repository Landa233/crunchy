use ndarray::Array;
use npyz::WriterBuilder;
use rs_to_npy::{array_wrapper::ArrayWrapper, dtypeable::DTypeable};
use z_n_gauge::backups::{
    backup_fragments::{
        BackupData, ExperimentParameters, LastState, PolyakovLoopsBackup, RebootSeed, RunInfo,
    },
    rng_gen_wrapper::RngGenWrapper,
};

fn main() {
    let last_state = LastState {
        edges: ArrayWrapper {
            data: Array::from_shape_vec([2, 2], vec![1; 4])
                .unwrap()
                .into_dyn(),
        },
        rng_gen: RngGenWrapper {
            rng_gen: rand_pcg::Pcg64Mcg::new(0),
        },
        performed_updates: 1,
        accepted_updates: 2,
    };

    let experiment_parameters = ExperimentParameters {
        shape: [3, 4, 5, 6],
        z_order: 7,
        beta: 8.0,
        lambda: 9.0,
        rng_seed: 10,
    };

    let reboot_seed = RebootSeed {
        last_state,
        experiment_parameters,
    };

    let backup_data = BackupData {
        polyakov_loops: ArrayWrapper {
            data: Array::from_shape_vec([2, 2], vec![1; 4])
                .unwrap()
                .into_dyn(),
        },
    };

    let run_info = RunInfo {
        recordings: 11,
        recording_skip: 12,
        recordings_until_backup: 13,
        backup_number: 14,
        recording_time: 15,
        run_id: 16,
    };

    let polyakov_backup = PolyakovLoopsBackup {
        reboot_seed,
        backup_data,
        run_info,
    };

    let backup_copy = polyakov_backup.clone();

    let file = std::fs::File::create("_round_trip_polyakov.npy").unwrap();
    let mut writer = npyz::WriteOptions::new()
        .dtype(polyakov_backup.generate_dtype())
        .writer(file)
        .begin_1d()
        .unwrap();

    writer.push(&polyakov_backup).unwrap();
    writer.finish().unwrap();

    let bytes = std::fs::read("_round_trip_polyakov.npy").unwrap();
    let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

    let deserialized_backup: Vec<PolyakovLoopsBackup> = reader.into_vec().unwrap();
    let deserialized = deserialized_backup.first().unwrap();

    assert_eq!(*deserialized, backup_copy);
}
