#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(generic_arg_infer)]

use std::fs;

use z_n_gauge::constants::SWEEP_ZORDER;
use z_n_gauge::measurements::measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr;
use z_n_gauge::measurements::measurement_types::polyakov_loops::PolyakovBackup;
use z_n_gauge::measurements::measurement_types::save_edges::SaveEdges;
use z_n_gauge::measurements::settings::MesaurementType;
use z_n_gauge::measurements::settings::ShedulerSettings;
use z_n_gauge::measurements::sheduler::phase_diagram_sweep;

fn main() {
    let file_name = "configs/sweep_settings.json";

    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let phase_diagram: ShedulerSettings =
        serde_json::from_str::<ShedulerSettings>(&contents).unwrap();

    let measurement_type = phase_diagram.measurement_type;

    match measurement_type {
        MesaurementType::Polyakovloops => {
            phase_diagram_sweep::<PolyakovBackup, SWEEP_ZORDER>(phase_diagram)
        }
        MesaurementType::SaveEdges => phase_diagram_sweep::<SaveEdges, SWEEP_ZORDER>(phase_diagram),
        MesaurementType::PloopPloopCorrCorr => {
            phase_diagram_sweep::<SavePloopPloopCorrCorr, SWEEP_ZORDER>(phase_diagram)
        } // No Fall Through
    }
}
