use rand_pcg::Pcg64Mcg;
use serde::{Deserialize, Serialize};

use super::experiment::ExperimentParameters;

#[derive(Debug, Serialize, Deserialize)]
pub struct LatticeBackup {
    pub experiment_parameters: ExperimentParameters,
    pub edges_flat_data: Vec<usize>,
    pub rng_gen: Pcg64Mcg,
    pub performed_updates: u64,
}
