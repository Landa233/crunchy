use crunchy::simulation::simulation::CubicalSimulation;
use std::ops::Deref;

use crate::gauge_fields::edges::EdgeField;
use crate::gauge_fields::lattice::ZNLatticeTypes;
use crate::gauge_fields::lattice::ZNParameters;
use crunchy::crarray::shape::Shape;
use crunchy::lattice::cubical_lattice::CubicalLattice;
use crunchy::lattice::fields::UpdateField;

use rand::Rng;

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone)]
pub struct Experiment<const ZORDER: usize> {
    pub sim: CubicalSimulation<4, ZNLatticeTypes<4, ZORDER>, ZNParameters<ZORDER>>,
    pub rng_gen: Pcg64Mcg,

    pub performed_updates: u64,
    pub accepted_updates: u64,
}

impl<const ZORDER: usize> Experiment<ZORDER> {
    pub fn new(shape: Shape<4>, sim_parameters: ZNParameters<ZORDER>) -> Self {
        let sim = CubicalSimulation::<4, ZNLatticeTypes<4, ZORDER>, ZNParameters<ZORDER>> {
            lattice: CubicalLattice::<4, ZNLatticeTypes<4, ZORDER>>::new(shape),
            sim_parameters,
        };

        // let rng_gen = Pcg64Mcg::seed_from_u64(1);
        let rng_gen = Pcg64Mcg::from_entropy();

        Experiment {
            sim,
            rng_gen,
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
