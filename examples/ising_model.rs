use std::fmt;

use colored::Colorize;
use crunchy::{
    crarray::shape::Shape,
    lattice::{
        ind::Ind,
        lattice::{EmptyField, Field, Lattice, LatticeTypes, SimParameter, UpdateField},
    },
};

#[derive(Debug, Copy, Clone, Default)]
struct Spin {
    up_down: bool,
}

impl fmt::Display for Spin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.up_down { "↑" } else { "↓" })
    }
}

impl Field for Spin {
    type LatticeMarker = SpinLattice;
}

impl UpdateField for Spin {
    fn update(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
        field: Self,
    ) -> Self {
        let old_field = grid.vertices[node_index].data;

        grid.vertices[node_index].data = field;

        old_field
    }

    fn energy(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) -> f32 {
        let shape = *grid.vertices.shape();

        let up = (Ind::new(node_index) + [0, 0, 1]) % shape;
        let down = (Ind::new(node_index) + [0, 0, shape[2] - 1]) % shape;
        let right = (Ind::new(node_index) + [0, 1, 0]) % shape;
        let left = (Ind::new(node_index) + [0, shape[1] - 1, 0]) % shape;

        let mut energy = 0.0;
        for dir in [up, down, right, left] {
            if grid.vertices[*dir].data.up_down == grid.vertices[node_index].data.up_down {
                energy += 1.0;
            } else {
                energy -= 1.0;
            }
        }
        -grid.sim_parameters.beta * energy
    }

    fn init() -> impl FnMut([usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1]) -> Self
    where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
    {
        |_node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1]| Spin {
            up_down: true,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct SpinLattice {}

#[derive(Debug, Copy, Clone, Default)]
struct IsingParameters {
    beta: f32,
}

impl SimParameter for IsingParameters {}

impl LatticeTypes for SpinLattice {
    const NDIM: usize = 2;

    type VertexType = Spin;
    type EdgeType = EmptyField<SpinLattice>;
    type FaceType = EmptyField<SpinLattice>;
    type CubeType = EmptyField<SpinLattice>;

    type SimParameterType = IsingParameters;
}

fn print_ising_model(lattice: &Lattice<2, SpinLattice>) {
    let shape = lattice.vertices.shape();
    for i in 0..shape[1] {
        for j in 0..shape[2] {
            if lattice.vertices[[0, i, j]].data.up_down {
                print!("{}", "■".green());
            } else {
                print!("{}", "■".red());
            }
        }
        println!();
    }
}

fn main() {
    let shape = Shape::new([50, 50]);

    let sim_params = IsingParameters { beta: 1. / 2.269 };

    let mut lattice = Lattice::<2, SpinLattice>::new(shape, sim_params);

    let mut f = Spin::init();

    for id in lattice.vertices.shape().iter() {
        lattice.vertices[id].data = f(id);
    }

    let vertex_shape = lattice.vertices.shape();
    let mut rng_gen = rand::thread_rng();

    for _ in 0..100000000 {
        let index = vertex_shape.random_index(&mut rng_gen);

        let new_field = Spin {
            up_down: !lattice.vertices[index].data.up_down,
        };

        Spin::metropolis_step(index, &mut lattice, new_field, &mut rng_gen);
    }

    print_ising_model(&lattice);
}
