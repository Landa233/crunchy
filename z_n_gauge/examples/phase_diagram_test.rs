#![feature(generic_const_exprs)]
use z_n_gauge::sheduler::{
    executor::HaltingCondition,
    phase_diagram::phase_diagram_sweep::{phase_diagram_sweep, PhaseDiagram, Range},
};

fn main() {
    const ZORDER: usize = 3;

    let phase_diagram = PhaseDiagram::<ZORDER> {
        beta_range: Range::new([0.49, 0.56], 24),
        lambda_range: Range::new([1.0, 1.0], 1),
        experiments_per_point: 1,
        number_of_threads: 8,
        halting_condition: HaltingCondition::Recordings(1000),
        recording_skip: 1,
        recordings_until_backup: 1234,
        lattice_shape: [4, 4, 4, 4],
        folder_name: "../../../nobackup/jhtb65/_recordings".to_string(),
    };

    phase_diagram_sweep(phase_diagram);
}
