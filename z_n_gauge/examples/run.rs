use std::fmt;

use colored::Colorize;
use crunchy::{
    crarray::shape::Shape,
    lattice::{
        ind::Ind,
        lattice::{EmptyField, Field, Lattice, LatticeTypes, SimParameter, UpdateField},
    },
};

#[derive(Debug, Clone, Copy, Default)]
struct EdgeField<const NDIM: usize, const ZORDER: usize> {
    phase: usize,
}

impl<const NDIM: usize, const ZORDER: usize> Field for EdgeField<NDIM, ZORDER> {
    type LatticeMarker = ZNLatticeTypes<NDIM, ZORDER>;
}

#[derive(Debug, Clone, Copy, Default)]
struct PlaquetteField<const NDIM: usize, const ZORDER: usize> {
    holonomy: usize,
}

impl<const NDIM: usize, const ZORDER: usize> Field for PlaquetteField<NDIM, ZORDER> {
    type LatticeMarker = ZNLatticeTypes<NDIM, ZORDER>;

    fn rebuild(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) {
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ZNLatticeTypes<const NDIM: usize, const ZORDER: usize> {}

impl<const NDIM: usize, const ZORDER: usize> LatticeTypes for ZNLatticeTypes<NDIM, ZORDER> {
    const NDIM: usize = NDIM;

    type VertexType = EdgeField<NDIM, ZORDER>;
    type EdgeType = PlaquetteField<NDIM, ZORDER>;
    type FaceType = EmptyField<ZNLatticeTypes<NDIM, ZORDER>>;
    type CubeType = EmptyField<ZNLatticeTypes<NDIM, ZORDER>>;

    type SimParameterType = ZNParameters;
}

#[derive(Debug, Clone, Copy, Default)]
struct ZNParameters {
    beta: f32,
}

impl SimParameter for ZNParameters {}

fn main() {
    let shape = Shape::new([50, 50]);

    // let sim_params = IsingParameters { beta: 1. / 2.269 };

    // let mut lattice = Lattice::<2, SpinLattice>::new(shape, sim_params);

    // let mut f = Spin::init();

    // for id in lattice.vertices.shape().iter() {
    //     lattice.vertices[id].data = f(id);
    // }

    // let vertex_shape = lattice.vertices.shape();
    // let mut rng_gen = rand::thread_rng();

    // for _ in 0..100000000 {
    //     let index = vertex_shape.random_index(&mut rng_gen);

    //     let new_field = Spin {
    //         up_down: !lattice.vertices[index].data.up_down,
    //     };

    //     Spin::metropolis_step(index, &mut lattice, new_field, &mut rng_gen);
    // }
}
