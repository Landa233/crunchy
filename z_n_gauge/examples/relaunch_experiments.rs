#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::{fs, path::Path};

use z_n_gauge::{
    constants::RELAUNCH_ZORDER,
    measurements::{
        measurement_types::{
            ploop_ploopcorr_corr::SavePloopPloopCorrCorr, polyakov_loops::PolyakovBackup,
            save_edges::SaveEdges,
        },
        settings::{MesaurementType, RelaunchSettings},
        sheduler::{experiments_to_relaunch, relaunch_experiments},
    },
};

fn main() {
    let file_name = "configs/relaunch_settings.json";

    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let relaunch_settings: RelaunchSettings = serde_json::from_str(&contents).unwrap();

    let mut number_of_threads = relaunch_settings.max_threads;

    let experiments = experiments_to_relaunch(&relaunch_settings.folder_path);

    number_of_threads = number_of_threads.min(experiments.0.len());

    let folder_path = Path::new(&relaunch_settings.folder_path);
    let measurement_type = relaunch_settings.measurement_type;

    match measurement_type {
        MesaurementType::Polyakovloops => {
            relaunch_experiments::<_, RELAUNCH_ZORDER, PolyakovBackup>(
                folder_path,
                number_of_threads,
            );
        }
        MesaurementType::SaveEdges => {
            relaunch_experiments::<_, RELAUNCH_ZORDER, SaveEdges>(folder_path, number_of_threads)
        }
        MesaurementType::PloopPloopCorrCorr => {
            relaunch_experiments::<_, RELAUNCH_ZORDER, SavePloopPloopCorrCorr>(
                folder_path,
                number_of_threads,
            );
        } // No Fall Through
    }
}
