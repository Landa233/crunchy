use std::{fs, io};

use ndarray::{Array, IxDyn};
use npyz::WriterBuilder;

use crate::experiment::{
    backup::backup::{backup_dtype, RebootSeed, RunInfo},
    experiment::Experiment,
};

#[derive(Clone)]
pub struct ExecutorParameters {
    pub recordings: u32,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
    pub run_id: u32,
    pub parent_path: String,
}

pub fn execute<const ZORDER: usize>(exec_par: ExecutorParameters, reboot_seed: RebootSeed) {
    assert!(ZORDER == reboot_seed.experiment_parameters.z_order as usize);

    let run_folder_name = format!("{}/{}", exec_par.parent_path, exec_par.run_id);
    fs::create_dir_all(&run_folder_name).unwrap();

    let mut experiment = Experiment::<ZORDER>::reboot_experiment(reboot_seed);

    let mut backup_number = 1;
    let mut recordings_to_go = exec_par.recordings;
    let recordings_until_backup = exec_par.recordings_until_backup;

    let shape = experiment.sim.shape;

    while recordings_to_go > 0 {
        let recordings = if recordings_to_go > recordings_until_backup {
            recordings_until_backup
        } else {
            recordings_to_go
        };

        let rec_shape = [recordings as usize, shape[0], shape[1], shape[2]];
        let zeros: Vec<u8> = vec![0; rec_shape.iter().product()];

        let mut polyakov_recordings: Array<u8, IxDyn> =
            Array::from_shape_vec(rec_shape, zeros).unwrap().into_dyn();

        for i in 0..recordings {
            experiment.record_experiment(&mut polyakov_recordings, i as usize);
            for _ in 0..exec_par.recording_skip {
                experiment.sweep();
            }
        }

        let now = chrono::Utc::now();
        let recording_time = now.timestamp();

        let run_info = RunInfo {
            recordings,
            recording_skip: exec_par.recording_skip,
            recordings_until_backup,
            backup_number,
            recording_time,
            run_id: exec_par.run_id,
        };

        let backup = experiment.backup(polyakov_recordings, run_info);

        let backup_folder_name = format!("{}/{}.npy", run_folder_name, backup_number);
        let mut file = io::BufWriter::new(fs::File::create(&backup_folder_name).unwrap());
        let mut writer = npyz::WriteOptions::new()
            .dtype(backup_dtype(&backup))
            .writer(&mut file)
            .begin_1d()
            .unwrap();

        writer.push(&backup).unwrap();
        writer.finish().unwrap();

        recordings_to_go -= recordings;
        backup_number += 1;
    }
}
