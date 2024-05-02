#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use std::env;
use std::fs;

use z_n_gauge::measurements::measurement_types::polyakov_loops::PolyakovBackup;
use z_n_gauge::measurements::measurement_types::save_edges::SaveEdges;
use z_n_gauge::measurements::sheduler::phase_diagram_sweep;
use z_n_gauge::measurements::sheduler::MesaurementType;
use z_n_gauge::measurements::sheduler::ShedulerSettings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file name as a command line argument");
        return;
    }

    env::var("CRUNCHY_ROOT_DIRECTORY").expect("env variable CRUNCHY_ROOT_DIRECTORY not set");

    let file_name = &args[1];

    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let phase_diagram: ShedulerSettings<3> = serde_json::from_str(&contents).unwrap();

    let measurement_type = phase_diagram.measurement_type;

    match measurement_type {
        MesaurementType::Polyakovloops => phase_diagram_sweep::<PolyakovBackup, 3>(phase_diagram),
        MesaurementType::SaveEdges => phase_diagram_sweep::<SaveEdges, 3>(phase_diagram),
        // No Fall Through
    }
}
