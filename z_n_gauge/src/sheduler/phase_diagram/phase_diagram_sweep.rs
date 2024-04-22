use std::{fmt::format, fs::File, io::Write, num::NonZeroUsize};

use chrono::prelude::*;
use crunchy::lattice;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    experiment::{
        backup::backup::{ExperimentParameters, RebootSeed},
        experiment::{generate_cosine, Experiment},
    },
    gauge_fields::lattice::ZNParameters,
    sheduler::executor::{execute, ExecutorParameters, HaltingCondition},
};

use super::zipper::archive;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PhaseDiagram<const ZORDER: usize> {
    pub beta_range: Range,
    pub lambda_range: Range,

    pub experiments_per_point: usize,

    pub number_of_threads: usize,
    pub halting_condition: HaltingCondition,
    pub recording_skip: usize,
    pub recordings_until_backup: usize,

    pub lattice_shapes: LatticeShapes,

    pub root_directory: String,

    pub cluster_settings: ClusterSettings,
}

impl<const ZORDER: usize> PhaseDiagram<ZORDER> {
    pub fn write_batch_file(&self, config_file: &str) {
        let file_path = "crunchy.sh";

        let ClusterSettings {
            cluster_time,
            ram,
            temporary_storage,
            email,
            path_to_executable,
            queue,
        } = self.cluster_settings.clone();

        let number_of_threads = self.number_of_threads;

        let content = format!(
            "#!/bin/bash\n\n# Request resources:\n\
            #SBATCH -c {}\n\
            #SBATCH --time={}-{}:{}:{}\n\
            #SBATCH --mem={}G\n\
            #SBATCH --tmp={}G\n\
            #SBATCH --mail-user={}\n\
            #SBATCH --mail-type=ALL\n\
            #SBATCH -p {}\n\n\n\
            #Commands to be run:\n\
            ./{} {}",
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
            path_to_executable,
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
    pub path_to_executable: String,
    pub queue: ClusterQueue,
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

pub fn phase_diagram_sweep<const ZORDER: usize>(phase_diagram: PhaseDiagram<ZORDER>) {
    let phase_diagram_json = phase_diagram.clone();

    let PhaseDiagram {
        beta_range,
        lambda_range,
        experiments_per_point,
        number_of_threads,
        halting_condition,
        recording_skip,
        recordings_until_backup,
        lattice_shapes,
        root_directory,
        cluster_settings,
    } = phase_diagram;

    let betas = return_steps(beta_range);
    let lambdas = return_steps(lambda_range);

    let mut experiments = vec![];

    let mut rng = rand::thread_rng();

    let folder_name = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();

    // let current_path = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();
    // let current_path = format!("{}/{}", root, current_path);

    // let run_path = current_path.clone();

    // Serialize phase_diagram_parameters to json
    let phase_diagram_parameters = serde_json::to_string_pretty(&phase_diagram_json).unwrap();
    let folder_path = format!("{}/{}", root_directory, folder_name);
    std::fs::create_dir_all(&folder_path).unwrap();
    std::fs::write(
        format!("{}/_phase_diagram_parameters.json", &folder_path),
        phase_diagram_parameters,
    )
    .unwrap();

    let format_float = |f: f32| -> String { format!("{:.2}", f).replace(".", "p") };

    let lattice_shapes: Vec<[usize; 4]> = lattice_shapes.into();

    let mut run_id = 1;
    for lattice_shape in lattice_shapes.iter() {
        for beta in betas.iter() {
            for lambda in lambdas.iter() {
                for _ in 0..experiments_per_point {
                    let sim_pars = ZNParameters {
                        beta: *beta,
                        cosines: generate_cosine(),
                        lambda: *lambda,
                    };

                    let rng_seed = rng.gen();

                    let experiment =
                        Experiment::<ZORDER>::new((*lattice_shape).into(), sim_pars, rng_seed);

                    let experiment_file_name = format!(
                        "{}/{}-L_{}_{}_{}_{}-b_{}-l_{}",
                        folder_path,
                        run_id,
                        lattice_shape[0],
                        lattice_shape[1],
                        lattice_shape[2],
                        lattice_shape[3],
                        format_float(*beta),
                        format_float(*lambda)
                    );
                    let executor_parameters = ExecutorParameters {
                        halting_condition,
                        recording_skip: recording_skip as u32,
                        recordings_until_backup: recordings_until_backup as u32,
                        run_id: run_id as u32,
                        parent_path: experiment_file_name,
                    };

                    experiments.push((executor_parameters, experiment.to_seed()));
                    run_id += 1;
                }
            }
        }
    }

    let experiments_per_thread = split_into_maximal_sublists(experiments, number_of_threads);

    // let chunk_size = experiments.len() / number_of_threads;

    // // Distribute experiments to threads
    // let experiments_per_thread: Vec<Vec<(ExecutorParameters, RebootSeed)>> =
    //     experiments.chunks(chunk_size).map(|s| s.into()).collect();

    // run the experiments in parallel threads
    let mut threads = vec![];
    experiments_per_thread.into_iter().for_each(|experiments| {
        threads.push(std::thread::spawn(move || {
            experiments
                .into_iter()
                .for_each(|(executor_parameters, reboot_seed)| {
                    execute::<ZORDER>(executor_parameters, reboot_seed);
                });
        }));
    });

    threads.into_iter().for_each(|t| t.join().unwrap());

    archive(folder_path);
}

fn split_into_maximal_sublists<T: Clone>(list: Vec<T>, number_of_chunks: usize) -> Vec<Vec<T>> {
    let quotient = list.len() / number_of_chunks;
    let remainder = list.len() % number_of_chunks;

    let mut chunk_sizes = vec![0; number_of_chunks];

    for i in 0..remainder {
        chunk_sizes[i] += 1;
    }

    for chunk in chunk_sizes.iter_mut() {
        *chunk += quotient;
    }

    let mut chunks = vec![];

    let mut list_iter = list.into_iter();
    for chunk_size in chunk_sizes {
        let chunk = list_iter.by_ref().take(chunk_size).collect();
        chunks.push(chunk);
    }

    chunks
}
