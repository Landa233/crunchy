use std::fmt;

use crunchy::{
    crarray::shape::Shape,
    lattice::lattice::{Field, Lattice, LatticeTypes, SimParameter, UpdateField},
};

#[derive(Debug, Copy, Clone, Default)]
struct Empty {}

impl fmt::Display for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty")
    }
}

impl Field for Empty {
    type LatticeMarker = SpinLattice;

    fn rebuild(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) {
        todo!()
    }
}

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

    fn rebuild(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) {
        todo!()
    }
}

impl UpdateField for Spin {
    fn update(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) -> Self {
        todo!()
    }

    fn set(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
        field: Self,
    ) {
        todo!()
    }

    fn energy(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) -> f32 {
        todo!()
    }

    fn init() -> impl FnMut([usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1]) -> Self
    where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
    {
        |node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1]| Spin {
            up_down: false,
        }
    }
}

struct SpinLattice {}

#[derive(Debug, Copy, Clone, Default)]
struct IsingParameters {
    temperature: f32,
}

impl SimParameter for IsingParameters {}

impl LatticeTypes for SpinLattice {
    const NDIM: usize = 2;

    type VertexType = Spin;
    type EdgeType = Empty;
    type FaceType = Empty;
    type CubeType = Empty;

    type SimParameterType = IsingParameters;
}

fn main() {
    let shape = Shape::new([2, 2]);

    let sim_params = IsingParameters { temperature: 1.0 };

    let mut lattice = Lattice::<2, SpinLattice>::new(shape, sim_params);

    let mut f = Spin::init();

    for id in lattice.vertices.shape().iter() {
        lattice.vertices[id].data = f(id);
    }

    println!("{}", lattice.vertices);
}
