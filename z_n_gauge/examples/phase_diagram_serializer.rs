#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use serde_json;
use z_n_gauge::measurements::backup::backup_trait::HaltingCondition;
use z_n_gauge::measurements::sheduler::ClusterQueue;
use z_n_gauge::measurements::sheduler::ClusterSettings;
use z_n_gauge::measurements::sheduler::ClusterTime;
use z_n_gauge::measurements::sheduler::LatticeShapes;
use z_n_gauge::measurements::sheduler::MesaurementType;
use z_n_gauge::measurements::sheduler::PhaseDiagram;
use z_n_gauge::measurements::sheduler::Range;

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
        temporary_storage: 100,
        email: "jhtb65@durham.ac.uk".to_string(),
        path_to_executable: "target/release/examples/run_sweep".to_string(),
        queue: ClusterQueue::Test,
    };

    let phase_diagram = PhaseDiagram::<ZORDER> {
        beta_range: Range::new([0.48, 0.52], 24),
        lambda_range: Range::new([1.0, 1.0], 1),
        experiments_per_point: 1,
        number_of_threads: 128,
        halting_condition: HaltingCondition::Recordings(20000),
        recording_skip: 1,
        recordings_until_backup: 5000,
        lattice_shapes: LatticeShapes::Sweep(8..13),
        root_directory: "../../../nobackup/jhtb65/_recordings".to_string(),
        // root_directory: "_test".to_string(),
        cluster_settings,
        measurement_type: MesaurementType::Polyakovloops,
    };

    // // Serialize the phase_diagram to a json file
    let json = serde_json::to_string(&phase_diagram).unwrap();

    let mut config_path = std::path::PathBuf::new();
    config_path.push("configs");
    config_path.push("phase_diagram.json");

    println!("{:?}", config_path);

    std::fs::write(config_path, json).unwrap();

    // Deserialize the phase_diagram from the json file

    let json = std::fs::read_to_string("configs/phase_diagram.json").unwrap();
    let phase_diagram: PhaseDiagram<ZORDER> = serde_json::from_str(&json).unwrap();

    // phase_diagram_sweep(phase_diagram);

    // archive("test/2024-04-22--15-21-51");
}
// folder_name: "../../../nobackup/jhtb65/_recordings".to_string(),
