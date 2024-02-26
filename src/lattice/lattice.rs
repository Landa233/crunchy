use std::fmt::Debug;

use crate::math_utils::binomial_coefficient;

trait LatticeTypes {
    type VertexType: Field;
    type EdgeType: Field;
    type FaceType: Field;
    type CubeType: Field;
}

trait Field: Debug + Copy + Clone + Default {}

struct Node<GraphConnections, FieldType: Field> {
    graph_connections: GraphConnections,
    data: FieldType,
}

struct VertexConnections<const NDIM: usize>
where
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
{
    edges: [u32; NDIM * 2],
    faces: [u32; 2 * (NDIM - 1)],
    cubes: [u32; 4 * binomial_coefficient(NDIM, 2)],
}

struct EdgeConnections<const NDIM: usize>
where
    [(); 2 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
{
    vertices: [u32; 2],
    faces: [u32; 2 * binomial_coefficient(NDIM, 2)],
    cubes: [u32; 4 * binomial_coefficient(NDIM - 1, 2)],
}

struct FaceConnections<const NDIM: usize>
where
    [(); 2 * (NDIM - 2)]:,
{
    vertices: [u32; 4],
    edges: [u32; 4],
    cubes: [u32; 2 * (NDIM - 2)],
}

struct CubeConnections<const NDIM: usize> {
    vertices: [u32; 8],
    edges: [u32; 12],
    faces: [u32; 6],
}

// struct Lattice<LT: LatticeTypes> {}
