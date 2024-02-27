use crunchy::{
    crarray::shape::Shape,
    lattice::lattice::{Field, Lattice, LatticeTypes},
};

struct TestLattice {}

#[derive(Debug, Copy, Clone, Default)]
struct Empty {}

impl Field for Empty {}

impl LatticeTypes for TestLattice {
    type VertexType = Empty;
    type EdgeType = Empty;
    type FaceType = Empty;
    type CubeType = Empty;
}

fn main() {
    let shape = Shape::new([3, 3]);

    let lattice = Lattice::<2, TestLattice>::new(shape);
}
