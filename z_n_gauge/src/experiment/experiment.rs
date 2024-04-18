use crunchy::lattice::fields::Field;
use crunchy::simulation::simulation::CubicalSimulation;
use ndarray::Array;
use ndarray::IxDyn;
use std::ops::Deref;

use crate::experiment::backup::backup::ExperimentParameters;
use crate::gauge_fields::cubes::MonopoleField;
use crate::gauge_fields::edges::EdgeField;
use crate::gauge_fields::lattice::ZNLatticeTypes;
use crate::gauge_fields::lattice::ZNParameters;
use crate::gauge_fields::plaquettes::PlaquetteField;
use crunchy::crarray::shape::Shape;
use crunchy::lattice::cubical_lattice::CubicalLattice;
use crunchy::lattice::fields::UpdateField;

use rand::Rng;

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use serde::Deserialize;
use serde::Serialize;

use super::backup::backup::BackUp;
use super::backup::backup::BackupData;
use super::backup::backup::LastState;
use super::backup::backup::RebootSeed;
use super::backup::backup::RunInfo;

#[derive(Clone)]
pub struct Experiment<const ZORDER: usize> {
    pub sim: CubicalSimulation<4, ZNLatticeTypes<4, ZORDER>, ZNParameters<ZORDER>>,
    pub rng_gen: Pcg64Mcg,
    pub rng_seed: u64,

    pub performed_updates: u64,
    pub accepted_updates: u64,
}

impl<const ZORDER: usize> Experiment<ZORDER> {
    pub fn new(shape: Shape<4>, sim_parameters: ZNParameters<ZORDER>, rng_seed: u64) -> Self {
        let sim = CubicalSimulation::<4, ZNLatticeTypes<4, ZORDER>, ZNParameters<ZORDER>> {
            lattice: CubicalLattice::<4, ZNLatticeTypes<4, ZORDER>>::new(shape),
            sim_parameters,
        };

        let rng_gen = Pcg64Mcg::seed_from_u64(rng_seed);
        // let rng_gen = Pcg64Mcg::from_entropy();

        Experiment {
            sim,
            rng_gen,
            rng_seed,
            performed_updates: 0,
            accepted_updates: 0,
        }
    }

    pub fn sweep(&mut self) {
        for edge_index in self.sim.edges.shape().iter() {
            let new_edge = EdgeField {
                phase: self.rng_gen.gen_range(0..ZORDER),
            };
            let update_accepted =
                EdgeField::metropolis_step(edge_index, &mut self.sim, new_edge, &mut self.rng_gen);

            if update_accepted.into() {
                self.accepted_updates += 1;
            }
            self.performed_updates += 1;
        }
    }

    pub fn to_seed(&self) -> RebootSeed {
        let edges = self
            .sim
            .edges
            .iter()
            .map(|edge| edge.data.phase as u8)
            .collect();

        let rng_gen = self.rng_gen.clone();
        let performed_updates = self.performed_updates;
        let accepted_updates = self.accepted_updates;

        let last_state = LastState {
            edges,
            rng_gen,
            performed_updates,
            accepted_updates,
        };

        let shape = self.shape;
        let shape = [
            shape[0] as u32,
            shape[1] as u32,
            shape[2] as u32,
            shape[3] as u32,
        ];

        let experiment_parameters = ExperimentParameters {
            shape,
            z_order: ZORDER as u32,
            beta: self.sim_parameters.beta,
            lambda: self.sim_parameters.lambda,
            rng_seed: self.rng_seed,
        };

        let reboot_seed = RebootSeed {
            last_state,
            experiment_parameters,
        };

        return reboot_seed;
    }

    pub fn backup(&self, polyakov_recordings: Array<u8, IxDyn>, run_info: RunInfo) -> BackUp {
        let reboot_seed = self.to_seed();

        let backup_data = BackupData {
            recorded_data: polyakov_recordings,
        };

        BackUp {
            reboot_seed,
            backup_data,
            run_info,
        }
    }

    pub fn record_experiment(&self, rec_array: &mut Array<u8, IxDyn>, rec_index: usize) {
        let [x_dim, y_dim, z_dim, t_dim] = *self.shape;

        for x in 0..x_dim {
            for y in 0..y_dim {
                for z in 0..z_dim {
                    let mut singe_loop = 0;
                    for t in 0..t_dim {
                        let a = self.sim.edges[[3, x, y, z, t]];
                        singe_loop += a.data.phase;
                    }
                    singe_loop %= ZORDER as usize;
                    rec_array[[rec_index, x, y, z]] = singe_loop as u8;
                }
            }
        }
    }

    pub fn reboot_experiment(reboot_seed: RebootSeed) -> Self {
        let shape = reboot_seed.experiment_parameters.shape;

        let zn_parameters = ZNParameters {
            beta: reboot_seed.experiment_parameters.beta,
            cosines: generate_cosine(),
            lambda: reboot_seed.experiment_parameters.lambda,
        };

        let mut experiment = Experiment::<ZORDER>::new(
            shape.into(),
            zn_parameters,
            reboot_seed.experiment_parameters.rng_seed,
        );

        experiment.rng_gen = reboot_seed.last_state.rng_gen.clone();

        experiment
            .sim
            .lattice
            .edges
            .iter_mut()
            .zip(reboot_seed.last_state.edges.iter())
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

        experiment.performed_updates = reboot_seed.last_state.performed_updates;
        experiment.accepted_updates = reboot_seed.last_state.accepted_updates;

        experiment
    }
}

pub fn generate_cosine<const ZORDER: usize>() -> [f32; ZORDER] {
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

impl<const ZORDER: usize> Deref for Experiment<ZORDER> {
    type Target = CubicalSimulation<4, ZNLatticeTypes<4, ZORDER>, ZNParameters<ZORDER>>;
    fn deref(&self) -> &Self::Target {
        &self.sim
    }
}

impl<const ZORDER: usize> PartialEq for Experiment<ZORDER> {
    fn eq(&self, other: &Self) -> bool {
        if self.rng_gen != other.rng_gen {
            return false;
        }
        if self.sim.sim_parameters != other.sim.sim_parameters {
            return false;
        }
        for edge_index in self.sim.edges.shape().iter() {
            if self.sim.edges[edge_index].data != other.sim.edges[edge_index].data {
                return false;
            }
        }
        for face_index in self.sim.faces.shape().iter() {
            if self.sim.faces[face_index].data != other.sim.faces[face_index].data {
                return false;
            }
        }
        for cube_index in self.sim.cubes.shape().iter() {
            if self.sim.cubes[cube_index].data != other.sim.cubes[cube_index].data {
                return false;
            }
        }

        if self.performed_updates != other.performed_updates {
            return false;
        }
        if self.accepted_updates != other.accepted_updates {
            return false;
        }

        true
    }
}
