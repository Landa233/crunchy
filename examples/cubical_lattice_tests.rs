use std::fmt;

use crunchy::{
    crarray::shape::Shape,
    lattice::lattice::{Field, Lattice, LatticeTypes},
};

struct TestLattice {}

#[derive(Debug, Copy, Clone, Default)]
struct Empty {}

const LATTICEDIM: usize = 3;

impl Field for Empty {
    type LatticeMarker = TestLattice;

    fn rebuild(
        node_index: [usize; LATTICEDIM + 1],
        grid: &mut Lattice<LATTICEDIM, Self::LatticeMarker>,
    ) {
        todo!()
    }
}

impl fmt::Display for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty")
    }
}

impl LatticeTypes for TestLattice {
    const NDIM: usize = LATTICEDIM;

    type VertexType = Empty;
    type EdgeType = Empty;
    type FaceType = Empty;
    type CubeType = Empty;
}

fn main() {
    let shape = Shape::new([2, 2, 2]);

    let lattice = Lattice::<3, TestLattice>::new(shape);

    println!("{}", lattice.vertices);
}
