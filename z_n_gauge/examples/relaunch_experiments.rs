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
        sheduler::relaunch_experiments,
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

    let folder_path = Path::new(&relaunch_settings.folder_path);
    let measurement_type = relaunch_settings.measurement_type;

    match measurement_type {
        MesaurementType::Polyakovloops => {
            relaunch_experiments::<_, RELAUNCH_ZORDER, PolyakovBackup>(folder_path, 8);
        }
        MesaurementType::SaveEdges => {
            relaunch_experiments::<_, RELAUNCH_ZORDER, SaveEdges>(folder_path, 8)
        }
        MesaurementType::PloopPloopCorrCorr => {
            relaunch_experiments::<_, RELAUNCH_ZORDER, SavePloopPloopCorrCorr>(folder_path, 8);
        } // No Fall Through
    }
}
