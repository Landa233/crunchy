use z_n_gauge::sheduler::phase_diagram::phase_diagram_sweep::{phase_diagram_sweep, PhaseDiagram};

fn main() {
    let phase_diagram = PhaseDiagram {
        beta_range: [0.4, 0.6],
        beta_steps: 5,
        lambda_range: [0.1, 1.5],
        lambda_steps: 3,
        experiments_per_point: 1,
        number_of_threads: 5,
        recordings: 5000,
        recording_skip: 1,
        recordings_until_backup: 1000,
        lattice_shape: [4, 4, 4, 4],
        folder_name: "_test".to_string(),
    };

    phase_diagram_sweep(phase_diagram);
}
