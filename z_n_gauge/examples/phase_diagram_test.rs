#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use z_n_gauge::sheduler::{
    executor::HaltingCondition,
    phase_diagram::phase_diagram_sweep::{
        phase_diagram_sweep, ClusterSettings, LatticeShapes, PhaseDiagram, Range,
    },
};

fn main() {
    const ZORDER: usize = 3;

    let phase_diagram = PhaseDiagram::<ZORDER> {
        beta_range: Range::new([0.01, 0.7], 128),
        lambda_range: Range::new([1.0, 1.0], 1),
        experiments_per_point: 1,
        number_of_threads: 128,
        halting_condition: HaltingCondition::Recordings(20000),
        recording_skip: 1,
        recordings_until_backup: 5000,
        lattice_shapes: LatticeShapes::Single([10, 10, 10, 10]),
        root_directory: "../../../nobackup/jhtb65/_recordings".to_string(),
        cluster_settings: ClusterSettings::default(),
    };

    phase_diagram_sweep(phase_diagram);
}
// folder_name: "../../../nobackup/jhtb65/_recordings".to_string(),
