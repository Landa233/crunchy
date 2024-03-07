use std::fmt;

use crunchy::{
    crarray::shape::Shape,
    lattice::lattice::{Field, Lattice, LatticeTypes},
};

struct TestLattice {}

#[derive(Debug, Copy, Clone, Default)]
struct Empty {}

impl Field for Empty {}

impl fmt::Display for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty")
    }
}

impl LatticeTypes for TestLattice {
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
