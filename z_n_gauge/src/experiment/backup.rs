use crunchy::{lattice::fields::Field, simulation::simulation::CubicalSimulation};
use rand_pcg::Pcg64Mcg;
use serde::{Deserialize, Serialize};

use crate::gauge_fields::{
    cubes::MonopoleField, edges, lattice::ZNParameters, plaquettes::PlaquetteField,
};

use super::experiment::{ExecutorParameters, Experiment};

#[derive(Debug, Serialize, Deserialize)]
pub struct LatticeBackup {
    pub experiment_parameters: ExecutorParameters,
    pub edges_flat_data: Vec<u8>,
    pub rng_gen: Pcg64Mcg,
    pub performed_updates: u64,
}

pub fn backup_experiment<const ZORDER: usize>(
    experiment: &Experiment<ZORDER>,
    experiment_parameters: ExecutorParameters,
    performed_updates: u64,
) -> LatticeBackup {
    let mut edges_flat_data = vec![];

    for edge in experiment.sim.lattice.edges.iter() {
        edges_flat_data.push(edge.data.phase.try_into().unwrap());
    }

    LatticeBackup {
        experiment_parameters,
        edges_flat_data,
        rng_gen: experiment.rng_gen.clone(),
        performed_updates,
    }
}

impl LatticeBackup {
    pub fn reboot_experiment<const ZORDER: usize>(&self) -> Experiment<ZORDER> {
        let ExecutorParameters {
            shape,
            z_order,
            beta,
            lambda,
            recordings,
            recording_skip,
            recordings_until_backup,
            rng_seed,
        } = self.experiment_parameters;

        let mut experiment = Experiment::<ZORDER>::new(
            shape.into(),
            ZNParameters {
                beta,
                cosines: generate_cosine(),
                lambda,
            },
        );

        experiment.rng_gen = self.rng_gen.clone();

        experiment
            .sim
            .lattice
            .edges
            .iter_mut()
            .zip(self.edges_flat_data.iter())
            .for_each(|(edge, phase)| {
                edge.data.phase = *phase as usize;
            });

        let plaquette_shape = experiment.sim.lattice.faces.shape();
        for plaquette_id in plaquette_shape.iter() {
            PlaquetteField::rebuild(plaquette_id, &mut experiment.sim);
        }

        let cube_shape = experiment.sim.lattice.cubes.shape();
        for cube_id in cube_shape.iter() {
            MonopoleField::rebuild(cube_id, &mut experiment.sim);
        }

        experiment
    }
}
fn generate_cosine<const ZORDER: usize>() -> [f32; ZORDER] {
    let pi = std::f32::consts::PI;
    let mut res = [0.0; ZORDER];

    // Generate perfectly symmetric cosine, otherwise this leads to symmetry breaking
    for i in 0..(ZORDER / 2 + 1) {
        res[i] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        if i != 0 {
            res[ZORDER - i] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        }
    }

    res
}
