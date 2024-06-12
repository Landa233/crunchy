use std::{
    fs::{self},
    path::Path,
};

use chrono::prelude::*;
use rand::Rng;

use crate::{
    experiment::experiment::{generate_cosine, Experiment},
    gauge_fields::lattice::ZNParameters,
    measurements::settings::{return_steps, HaltingCondition},
};

use super::{
    backup::backup_trait::{BackUp, ExecutorParameters},
    settings::ShedulerSettings,
};

pub fn phase_diagram_sweep<BackUpType: BackUp, const ZORDER: usize>(
    phase_diagram: ShedulerSettings,
) {
    let phase_diagram_json = phase_diagram.clone();

    let ShedulerSettings {
        beta_range,
        lambda_range,
        experiments_per_point,
        number_of_threads,
        halting_condition,
        recording_skip,
        recordings_until_backup,
        lattice_shapes,
        ..
    } = phase_diagram;

    let betas = return_steps(beta_range);
    let lambdas = return_steps(lambda_range);

    let mut experiments = vec![];

    let mut rng = rand::thread_rng();

    let folder_name = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();

    use std::env;
    let root_directory =
        env::var("CRUNCHY_ROOT_DIRECTORY").expect("env variable CRUNCHY_ROOT_DIRECTORY not set");

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
                        backup_number: 1,
                    };

                    experiments.push((executor_parameters, experiment.to_seed()));
                    run_id += 1;
                }
            }
        }
    }

    let experiments_per_thread = split_into_maximal_sublists(experiments, number_of_threads);

    // run the experiments in parallel threads
    let mut threads = vec![];
    experiments_per_thread.into_iter().for_each(|experiments| {
        threads.push(std::thread::spawn(move || {
            experiments
                .into_iter()
                .for_each(|(executor_parameters, reboot_seed)| {
                    // execute::<ZORDER>(executor_parameters, reboot_seed);
                    BackUpType::execute::<ZORDER>(executor_parameters, reboot_seed)
                });
        }));
    });

    threads.into_iter().for_each(|t| t.join().unwrap());

    println!("{}", "Completed all experiments");

    // archive::<BackUpType, _>(folder_path);
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

pub fn relaunch_experiments<P, const ZORDER: usize, BackUpType: BackUp>(
    experiment_directory: P,
    number_of_threads: usize,
) where
    P: AsRef<Path>,
{
    let (experiments_to_reboot, sheduler_settings) =
        experiments_to_relaunch::<P>(experiment_directory);

    let ShedulerSettings {
        halting_condition,
        recordings_until_backup,
        ..
    } = sheduler_settings;

    let recordings_goal = match halting_condition {
        HaltingCondition::Recordings(rec) => rec,
        HaltingCondition::Time(_) => panic!("Halting condition must be recordings to relaunch"),
    };

    let mut experiments_to_execute = vec![];

    for (file_name, max_backup_id) in experiments_to_reboot {
        let last_backup_file = Path::new(&file_name).join(format!("{}.npy", max_backup_id));

        // get backup from the last backup file
        let backup = BackUpType::from_file(last_backup_file);

        let reboot_seed = backup.reboot_seed();
        let run_info = backup.run_info();

        let recordings_to_go = recordings_goal - (max_backup_id * recordings_until_backup as u32);

        let executor_parameters = ExecutorParameters {
            halting_condition: HaltingCondition::Recordings(recordings_to_go),
            recording_skip: sheduler_settings.recording_skip as u32,
            recordings_until_backup: sheduler_settings.recordings_until_backup as u32,
            run_id: run_info.run_id,
            parent_path: file_name,
            backup_number: max_backup_id + 1,
        };

        experiments_to_execute.push((executor_parameters, reboot_seed));
    }

    println!(
        "Relaunching {} experiment(s).",
        experiments_to_execute.len()
    );

    for (executor_parameters, _) in &experiments_to_execute {
        println!(
            "Launching: {}, with {} left to go, starting from backup {}.",
            executor_parameters.parent_path,
            executor_parameters.halting_condition,
            executor_parameters.backup_number
        );
    }

    let experiments_per_thread =
        split_into_maximal_sublists(experiments_to_execute, number_of_threads);

    let mut threads = vec![];
    experiments_per_thread.into_iter().for_each(|experiments| {
        threads.push(std::thread::spawn(move || {
            experiments
                .into_iter()
                .for_each(|(executor_parameters, reboot_seed)| {
                    BackUpType::execute::<ZORDER>(executor_parameters, reboot_seed)
                });
        }));
    });

    threads.into_iter().for_each(|t| t.join().unwrap());

    println!("{}", "Completed all experiments");
}

pub fn experiments_to_relaunch<P>(experiment_directory: P) -> (Vec<(String, u32)>, ShedulerSettings)
where
    P: AsRef<std::path::Path>,
{
    // Get all the experiment directories
    let experiment_directories = std::fs::read_dir(&experiment_directory).unwrap();

    let mut experiments = vec![];

    // Iterate over the experiment directories
    for experiment_directory in experiment_directories {
        let experiment_directory = experiment_directory.unwrap().path();

        // Check if the directory is a directory
        if !experiment_directory.is_dir() {
            continue;
        }
        experiments.push(experiment_directory);
    }

    let settings_file_name = experiment_directory
        .as_ref()
        .join("_phase_diagram_parameters.json");
    let contents = match fs::read_to_string(settings_file_name) {
        Ok(contents) => contents,
        Err(err) => {
            panic!("Error reading file: {}", err);
        }
    };

    // Read the phase_diagram_parameters.json file
    let phase_diagram: ShedulerSettings =
        serde_json::from_str::<ShedulerSettings>(&contents).unwrap();

    let recordings_goal = match phase_diagram.halting_condition {
        HaltingCondition::Recordings(rec) => rec,
        HaltingCondition::Time(_) => panic!("Halting condition must be recordings to relaunch"),
    };

    let recordings_until_backup = phase_diagram.recordings_until_backup;

    let mut experiments_to_reboot = vec![];

    // loop over the experiments and relaunch them
    for experiment_directory in experiments {
        // loop through files in the experiment directory
        let experiment_files = std::fs::read_dir(&experiment_directory).unwrap();

        let mut backup_files = vec![];

        for experiment_file in experiment_files {
            let experiment_file = experiment_file.unwrap().path();

            if let Some(extension) = experiment_file.extension() {
                if extension == "npy" {
                    backup_files.push(experiment_file);
                }
            }
        }

        let mut max_backup_id = 0;
        for backup_file in backup_files {
            let file_name = backup_file.file_name().unwrap().to_str().unwrap();
            let run_id = file_name.split('.').next().unwrap().parse::<u32>().unwrap();

            if run_id > max_backup_id {
                max_backup_id = run_id;
            }
        }

        if (max_backup_id * recordings_until_backup as u32) >= recordings_goal {
            continue;
        };

        experiments_to_reboot.push((
            experiment_directory.to_str().unwrap().to_owned(),
            max_backup_id,
        ));
    }

    (experiments_to_reboot, phase_diagram)
}
