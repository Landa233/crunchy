use crunchy::crarray::ind::Ind;
use crunchy::lattice::fields::Field;
use crunchy::simulation::simulation::CubicalSimulation;
use ndarray::arr1;

use ndarray::s;
use ndarray::Array;
use ndarray::Dim;
use ndarray::Ix1;
use rs_to_npy::array_wrapper::ArrayWrapper;
use std::ops::Deref;

use crate::gauge_fields::cubes::MonopoleField;
use crate::gauge_fields::edges::EdgeField;
use crate::gauge_fields::lattice::ZNLatticeTypes;
use crate::gauge_fields::lattice::ZNParameters;
use crate::gauge_fields::plaquettes::PlaquetteField;

use crate::measurements::backup::backup_fragments::ExperimentParameters;
use crate::measurements::backup::backup_fragments::LastState;
use crate::measurements::backup::backup_fragments::RebootSeed;
use crate::measurements::rng_gen_wrapper::RngGenWrapper;
use crunchy::crarray::shape::Shape;
use crunchy::lattice::cubical_lattice::CubicalLattice;
use crunchy::lattice::fields::UpdateField;

use rand::Rng;

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

// Dimensions of lattice are [x,y,z,t]
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
        let edges: Vec<u8> = self
            .sim
            .edges
            .iter()
            .map(|edge| edge.data.phase as u8)
            .collect();

        let edges = ArrayWrapper {
            data: Array::from_vec(edges).into_dyn(),
        };

        let rng_gen = self.rng_gen.clone();
        let performed_updates = self.performed_updates;
        let accepted_updates = self.accepted_updates;

        let last_state = LastState {
            edges,
            rng_gen: RngGenWrapper { rng_gen },
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

        experiment.rng_gen = reboot_seed.last_state.rng_gen.rng_gen.clone();

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

fn calculate_distance_correlator(experiment: &Experiment<3>, d: usize) -> [f32; 2] {
    let mut roots_of_unity = vec![];

    let angle = 2.0 * std::f32::consts::PI / 3.0;
    roots_of_unity.push(arr1(&[1.0, 0.0]));
    roots_of_unity.push(arr1(&[angle.cos(), angle.sin()]));
    roots_of_unity.push(arr1(&[angle.cos(), -angle.sin()]));

    let mut counter = 0;
    let mut total_correlator = [0.0, 0.0];

    let mut ds = vec![];
    for i in 0..4 {
        let mut d_vec = [0; 4];
        d_vec[i] = d;
        ds.push(Ind::<4>::new(d_vec));
    }

    for d in ds {
        for x in 0..experiment.sim.faces.shape()[1] {
            for y in 0..experiment.sim.faces.shape()[2] {
                for z in 0..experiment.sim.faces.shape()[3] {
                    for t in 0..experiment.sim.faces.shape()[4] {
                        let x = Ind::<4>::new([x, y, z, t]);
                        let x_d = ((x + d) % *experiment.sim.shape).prepend(0);
                        let x = x.prepend(0);

                        let faces = &experiment.sim.faces;
                        let pp_correlator =
                            (3 + faces[*x].data.holonomy - faces[*x_d].data.holonomy) % 3;

                        total_correlator[0] += roots_of_unity[pp_correlator][0];
                        total_correlator[1] += roots_of_unity[pp_correlator][1];

                        counter += 1;
                    }
                }
            }
        }
    }

    total_correlator[0] /= counter as f32;
    total_correlator[1] /= counter as f32;

    total_correlator
}

pub fn calculate_correlators(experiment: &Experiment<3>) -> Vec<[f32; 2]> {
    let l = experiment.sim.shape[0];

    let mut correlators = vec![];
    for d in 0..l {
        correlators.push(calculate_distance_correlator(experiment, d));
    }

    return correlators;
    // panic!()
}
