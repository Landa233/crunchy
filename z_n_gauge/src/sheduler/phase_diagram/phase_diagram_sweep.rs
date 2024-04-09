use std::fmt::format;

use chrono::prelude::*;
use rand::Rng;
use serde::Serialize;

use crate::{
    experiment::{
        backup::backup::{ExperimentParameters, RebootSeed},
        experiment::{generate_cosine, Experiment},
    },
    gauge_fields::lattice::ZNParameters,
    sheduler::executor::{execute, ExecutorParameters},
};

// use super::executor::{execute, ExecutorParameters};

#[derive(Clone, Serialize)]
pub struct PhaseDiagram {
    pub beta_range: [f32; 2],
    pub beta_steps: usize,

    pub lambda_range: [f32; 2],
    pub lambda_steps: usize,

    pub experiments_per_point: usize,

    pub number_of_threads: usize,
    pub recordings: usize,
    pub recording_skip: usize,
    pub recordings_until_backup: usize,

    pub lattice_shape: [usize; 4],

    pub folder_name: String,
}

pub fn phase_diagram_sweep(phase_diagram: PhaseDiagram) {
    let phase_diagram_json = phase_diagram.clone();

    let PhaseDiagram {
        beta_range,
        beta_steps,
        lambda_range,
        lambda_steps,
        experiments_per_point,
        number_of_threads,
        recordings,
        recording_skip,
        recordings_until_backup,
        lattice_shape,
        folder_name,
    } = phase_diagram;

    let delta_beta = (beta_range[1] - beta_range[0]) / (beta_steps - 1) as f32;
    let delta_lambda = (lambda_range[1] - lambda_range[0]) / (lambda_steps - 1) as f32;

    let betas = (0..beta_steps)
        .into_iter()
        .map(|i| beta_range[0] + i as f32 * delta_beta)
        .collect::<Vec<f32>>();

    let lambdas = (0..lambda_steps)
        .into_iter()
        .map(|i| lambda_range[0] + i as f32 * delta_lambda)
        .collect::<Vec<f32>>();

    let mut experiments = vec![];

    let mut rng = rand::thread_rng();

    let root = folder_name;
    let current_path = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();
    let current_path = format!("{}/{}", root, current_path);

    // Serialize phase_diagram_parameters to json
    let phase_diagram_parameters = serde_json::to_string_pretty(&phase_diagram_json).unwrap();
    std::fs::create_dir_all(&current_path).unwrap();
    std::fs::write(
        format!("{}/_phase_diagram_parameters.json", current_path),
        phase_diagram_parameters,
    )
    .unwrap();

    let format_float = |f: f32| -> String { format!("{:.2}", f).replace(".", "p") };

    let mut i = 1;
    for beta in betas.iter() {
        for lambda in lambdas.iter() {
            for _ in 0..experiments_per_point {
                let sim_pars = ZNParameters {
                    beta: *beta,
                    cosines: generate_cosine(),
                    lambda: *lambda,
                };

                let rng_seed = rng.gen();

                let experiment = Experiment::<3>::new(lattice_shape.into(), sim_pars, rng_seed);

                let experiment_file_name = format!(
                    "{}/{}-b_{}-l_{}",
                    current_path,
                    i,
                    format_float(*beta),
                    format_float(*lambda)
                );
                let executor_parameters = ExecutorParameters {
                    recordings: recordings as u32,
                    recording_skip: recording_skip as u32,
                    recordings_until_backup: recordings_until_backup as u32,
                    run_id: i as u32,
                    parent_path: experiment_file_name,
                };

                experiments.push((executor_parameters, experiment.to_seed()));
                i += 1;
            }
        }
    }

    let chunk_size = experiments.len() / number_of_threads;

    // Distribute experiments to threads
    let experiments_per_thread: Vec<Vec<(ExecutorParameters, RebootSeed)>> =
        experiments.chunks(chunk_size).map(|s| s.into()).collect();

    // run the experiments in parallel threads
    let mut threads = vec![];
    experiments_per_thread.into_iter().for_each(|experiments| {
        threads.push(std::thread::spawn(move || {
            experiments
                .into_iter()
                .for_each(|(executor_parameters, reboot_seed)| {
                    execute::<3>(executor_parameters, reboot_seed);
                });
        }));
    });

    threads.into_iter().for_each(|t| t.join().unwrap());
}
