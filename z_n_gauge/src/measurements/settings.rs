use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::{fmt, fs::File, io::Write, num::NonZeroUsize, ops::Deref};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShedulerSettings {
    pub beta_range: Range,
    pub lambda_range: Range,

    pub experiments_per_point: usize,

    pub number_of_threads: usize,
    pub halting_condition: HaltingCondition,
    pub recording_skip: usize,
    pub recordings_until_backup: usize,

    pub lattice_shapes: LatticeShapes,

    pub cluster_settings: ClusterSettings,

    pub measurement_type: MesaurementType,

    pub z_order: usize,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub enum HaltingCondition {
    Recordings(u32),
    Time(DurationWrapper),
}

// impl Display for HaltingCondition
impl fmt::Display for HaltingCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaltingCondition::Recordings(recordings) => write!(f, "Recordings: {}", recordings),
            HaltingCondition::Time(time) => write!(f, "Time: {}", time.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DurationWrapper(pub Duration);

impl serde::Serialize for DurationWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.num_seconds())
    }
}

impl<'de> serde::Deserialize<'de> for DurationWrapper {
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

#[derive(Clone, Serialize, Deserialize, Debug, Copy)]
pub enum MesaurementType {
    Polyakovloops,
    SaveEdges,
    PloopPloopCorrCorr,
}

impl ShedulerSettings {
    pub fn write_batch_file(&self, config_file: &str) {
        let file_path = "crunchy.sh";

        let ClusterSettings {
            cluster_time,
            ram,
            temporary_storage,
            email,
            // path_to_executable,
            queue,
        } = self.cluster_settings.clone();

        let number_of_threads = self.number_of_threads;

        let content = format!(
            "#!/bin/bash\n\n# Request resources:\n\
            #SBATCH -c {}\n\
            #SBATCH --time={}-{}:{}:{}\n\
            #SBATCH --mem={}G\n\
            #SBATCH --gres=tmp:{}G\n\
            #SBATCH --mail-user={}\n\
            #SBATCH --mail-type=ALL\n\
            #SBATCH -p {}\n\n\n\
            #Commands to be run:\n\
            cargo run --manifest-path z_n_gauge/Cargo.toml --release --example run_sweep {}",
            number_of_threads,
            cluster_time.days,
            cluster_time.hours,
            cluster_time.minutes,
            cluster_time.seconds,
            ram,
            temporary_storage,
            email,
            match queue {
                ClusterQueue::Test => "test",
                ClusterQueue::Shared => "shared",
            },
            // path_to_executable,
            config_file
        );

        let mut file = File::create(file_path).unwrap();

        file.write_all(content.as_bytes()).unwrap();
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ClusterSettings {
    pub cluster_time: ClusterTime,
    pub ram: usize,               // in GB
    pub temporary_storage: usize, // in GB
    pub email: String,
    // pub path_to_executable: String,
    pub queue: ClusterQueue,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RelaunchSettings {
    pub folder_path: String,
    pub max_threads: usize,
    pub measurement_type: MesaurementType,
    pub cluster_settings: ClusterSettings,
    pub z_order: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClusterQueue {
    Test,
    Shared,
}

impl Default for ClusterQueue {
    fn default() -> Self {
        ClusterQueue::Shared
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ClusterTime {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum LatticeShapes {
    Single([usize; 4]),
    Custom(Vec<[usize; 4]>),
    Sweep(std::ops::Range<usize>),
    Evens([usize; 2]),
}

impl From<LatticeShapes> for Vec<[usize; 4]> {
    fn from(lattice_shape: LatticeShapes) -> Self {
        match lattice_shape {
            LatticeShapes::Single(shape) => vec![shape],
            LatticeShapes::Custom(shapes) => shapes,
            LatticeShapes::Sweep(range) => {
                let mut shapes = vec![];
                for i in range {
                    shapes.push([i, i, i, i]);
                }
                shapes
            }
            LatticeShapes::Evens([start, end]) => {
                if start % 2 != 0 || end % 2 != 0 {
                    panic!("Start and end of the range must be even numbers");
                }
                let mut shapes = vec![];
                for i in (start..=end).step_by(2) {
                    shapes.push([i, i, i, i]);
                }
                shapes
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Range(pub [f32; 2], pub NonZeroUsize);

impl Range {
    pub fn new(range: [f32; 2], steps: usize) -> Self {
        Self(range, NonZeroUsize::new(steps).unwrap())
    }
}

pub fn return_steps(range: Range) -> Vec<f32> {
    match range {
        Range([a, b], steps) => {
            let steps = steps.get();
            if steps == 1 {
                return vec![a];
            }

            let delta = (b - a) / (steps - 1) as f32;
            (0..steps)
                .into_iter()
                .map(|i| a + i as f32 * delta)
                .collect::<Vec<f32>>()
        }
    }
}
