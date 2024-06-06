use crunchy::{
    crarray::shape::{Shape, ShapeIterator},
    lattice::{cubical_lattice::CubicalLattice, fields::UpdateField},
    simulation::simulation::CubicalSimulation,
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use z_n_gauge::gauge_fields::{
    edges::EdgeField,
    lattice::{ZNLatticeTypes, ZNParameters},
};

pub struct Simulation2D<const ZORDER: usize> {
    pub sim: CubicalSimulation<2, ZNLatticeTypes<2, ZORDER>, ZNParameters<ZORDER>>,
    pub edge_iterator: ShapeIterator<3>,
    pub rng_gen: Pcg64Mcg,
}

#[derive(Debug)]
pub struct Package2D {
    pub edges: Vec<([usize; 3], usize)>,
    pub plaquettes: Vec<([usize; 3], usize)>,
}

impl<const ZORDER: usize> Simulation2D<ZORDER> {
    pub fn new(shape: Shape<2>, sim_parameters: ZNParameters<ZORDER>) -> Self {
        let sim = CubicalSimulation::<2, ZNLatticeTypes<2, ZORDER>, ZNParameters<ZORDER>> {
            lattice: CubicalLattice::<2, ZNLatticeTypes<2, ZORDER>>::new(shape),
            sim_parameters,
        };

        let edge_iterator: ShapeIterator<3> = sim.edges.shape().iter();
        let rng_gen = Pcg64Mcg::from_entropy();

        Self {
            sim,
            edge_iterator,
            rng_gen,
        }
    }

    pub fn edge_update(&mut self) -> Option<Package2D> {
        let next_edge = self.edge_iterator.next();

        let next_edge = match next_edge {
            Some(edge) => edge,
            None => {
                self.edge_iterator = self.sim.edges.shape().iter();
                self.edge_iterator.next().unwrap()
            }
        };

        let update_attempt = EdgeField {
            phase: self.rng_gen.gen_range(0..ZORDER),
        };

        let update_result =
            EdgeField::metropolis_step(next_edge, &mut self.sim, update_attempt, &mut self.rng_gen);

        if update_result.into() {
            let mut plaquettes = vec![];
            for p in self.sim.edges[next_edge].graph_connections.faces.iter() {
                plaquettes.push((*p, self.sim.faces[*p].data.holonomy));
            }

            let package = Package2D {
                edges: vec![(next_edge, self.sim.edges[next_edge].data.phase)],
                plaquettes,
            };
            return Some(package);
        }

        return None;
    }
}
