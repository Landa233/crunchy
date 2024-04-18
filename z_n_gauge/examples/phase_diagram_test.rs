#![feature(generic_const_exprs)]
use chrono::Duration;
use z_n_gauge::sheduler::{
    executor::HaltingCondition,
    phase_diagram::phase_diagram_sweep::{phase_diagram_sweep, PhaseDiagram, Range},
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
        recordings_until_backup: 3000,
        lattice_shape: [10, 10, 10, 10],
        folder_name: "../../../nobackup/jhtb65/_recordings".to_string(),
    };

    phase_diagram_sweep(phase_diagram);
}
// folder_name: "../../../nobackup/jhtb65/_recordings".to_string(),
