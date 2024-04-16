use std::num::NonZeroUsize;

use z_n_gauge::sheduler::{
    executor::HaltingCondition,
    phase_diagram::phase_diagram_sweep::{phase_diagram_sweep, return_steps, PhaseDiagram, Range},
};

fn main() {
    const ZORDER: usize = 7;

    let phase_diagram = PhaseDiagram::<ZORDER> {
        beta_range: Range::new([0.0, 2.5], 24),
        lambda_range: Range::new([0.0, 0.0], 1),
        experiments_per_point: 1,
        number_of_threads: 8,
        halting_condition: HaltingCondition::Recordings(5000),
        recording_skip: 1,
        recordings_until_backup: 1234,
        lattice_shape: [6, 6, 6, 6],
        folder_name: "_test".to_string(),
    };

    phase_diagram_sweep(phase_diagram);

    let r = Range([0.0, 1.0], NonZeroUsize::new(2).unwrap());
    println!("{:?}", return_steps(r));
}
