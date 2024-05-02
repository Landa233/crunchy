#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use serde_json;
use z_n_gauge::measurements::backup::backup_trait::HaltingCondition;
use z_n_gauge::measurements::sheduler::ClusterQueue;
use z_n_gauge::measurements::sheduler::ClusterSettings;
use z_n_gauge::measurements::sheduler::ClusterTime;
use z_n_gauge::measurements::sheduler::LatticeShapes;
use z_n_gauge::measurements::sheduler::MesaurementType;

use z_n_gauge::measurements::sheduler::Range;
use z_n_gauge::measurements::sheduler::ShedulerSettings;

fn main() {
    const ZORDER: usize = 3;

    let cluster_settings = ClusterSettings {
        cluster_time: ClusterTime {
            days: 0,
            hours: 3,
            minutes: 0,
            seconds: 0,
        },
        ram: 50,
        temporary_storage: 1,
        email: "jhtb65@durham.ac.uk".to_string(),
        // path_to_executable: "target/release/examples/run_sweep".to_string(),
        queue: ClusterQueue::Shared,
    };

    let sheduler_settings = ShedulerSettings::<ZORDER> {
        beta_range: Range::new([0.508, 0.508], 1),
        lambda_range: Range::new([1.0, 1.0], 1),
        experiments_per_point: 1,
        number_of_threads: 4,
        halting_condition: HaltingCondition::Recordings(20_000),
        recording_skip: 1,
        recordings_until_backup: 5_000,
        lattice_shapes: LatticeShapes::Sweep(8..13),
        // root_directory: "../../../nobackup/jhtb65/_recordings".to_string(),
        // root_directory: "_test".to_string(),
        cluster_settings,
        measurement_type: MesaurementType::SaveEdges,
    };

    // // Serialize the phase_diagram to a json file
    let json = serde_json::to_string(&sheduler_settings).unwrap();

    let mut config_path = std::path::PathBuf::new();
    config_path.push("configs");
    config_path.push("phase_diagram.json");

    println!("{:?}", config_path);

    std::fs::write(config_path, json).unwrap();

    // Deserialize the phase_diagram from the json file

    let json = std::fs::read_to_string("configs/phase_diagram.json").unwrap();
    let _phase_diagram: ShedulerSettings<ZORDER> = serde_json::from_str(&json).unwrap();

    // phase_diagram_sweep(phase_diagram);

    // archive("test/2024-04-22--15-21-51");
}
// folder_name: "../../../nobackup/jhtb65/_recordings".to_string(),
