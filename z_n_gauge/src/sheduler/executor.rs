use std::{fs, io, ops::Deref, time::Instant};

use chrono::{Duration, Utc};
use ndarray::{Array, IxDyn};
use npyz::WriterBuilder;
use serde::{Deserialize, Serialize};

use crate::experiment::{
    backup::backup::{backup_dtype, RebootSeed, RunInfo},
    experiment::Experiment,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExecutorParameters {
    pub halting_condition: HaltingCondition,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
    pub run_id: u32,
    pub parent_path: String,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum HaltingCondition {
    Recordings(u32),
    Time(DurationWrapper),
}

#[derive(Clone, Copy, Debug)]
pub struct DurationWrapper(pub Duration);

impl Serialize for DurationWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.num_seconds())
    }
}

impl<'de> Deserialize<'de> for DurationWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;
        Ok(Self(Duration::seconds(seconds)))
    }
}

impl Deref for DurationWrapper {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn execute<const ZORDER: usize>(exec_par: ExecutorParameters, reboot_seed: RebootSeed) {
    assert!(ZORDER == reboot_seed.experiment_parameters.z_order as usize);

    println!("{:?}", exec_par.run_id);

    fs::create_dir_all(&exec_par.parent_path).unwrap();

    let mut experiment = Experiment::<ZORDER>::reboot_experiment(reboot_seed);

    let mut backup_number = 1;

    let recordings_until_backup = exec_par.recordings_until_backup;

    let shape = experiment.sim.shape;

    let (max_recordings, duration) = match exec_par.halting_condition {
        HaltingCondition::Recordings(rec) => (rec, Duration::weeks(20000000)),
        HaltingCondition::Time(duration) => (2_u32.pow(31), *duration),
    };
    let start_time = Utc::now();

    let mut recordings_counter = 0;

    while recordings_counter < max_recordings && Utc::now() - start_time < duration {
        let recordings = if max_recordings - recordings_counter >= recordings_until_backup {
            recordings_until_backup
        } else {
            max_recordings - recordings_counter
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

        let backup_folder_name = format!("{}/{}.npy", exec_par.parent_path, backup_number);
        let mut file = io::BufWriter::new(fs::File::create(&backup_folder_name).unwrap());
        let mut writer = npyz::WriteOptions::new()
            .dtype(backup_dtype(&backup))
            .writer(&mut file)
            .begin_1d()
            .unwrap();

        writer.push(&backup).unwrap();
        writer.finish().unwrap();

        recordings_counter += recordings;
        backup_number += 1;
    }
}
