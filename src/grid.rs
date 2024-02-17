use std::fmt::Debug;

use crate::math_utils::binomial_coefficient;

pub struct Node<GraphConnections, NodeData: LatticeData> {
    graph_connections: GraphConnections,
    data: NodeData,
}

pub struct EdgeConnections<const NDIM: usize>
where
    [(); 2 * NDIM - 2]:,
    [(); NDIM + 1]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
{
    plaquettes: [[usize; NDIM + 1]; 2 * NDIM - 2],
    cube_ids: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM - 1, 2)],
}

pub struct PlaquetteConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 2)]:,
{
    edges: [[usize; NDIM + 1]; 4],
    cubes: [[usize; NDIM + 1]; 2 * (NDIM - 2)],
}

pub struct CubeConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
{
    edges: [[usize; NDIM + 1]; 12],
    plaquettes: [[usize; NDIM + 1]; 6],
}

pub trait LatticeData: Copy + Clone + Debug {
    // pub fn update()
}

pub trait LatticeTypes {
    type EdgeData: LatticeData;
    type PlaquetteData: LatticeData;
    type CubeData: LatticeData;
    type NodeData: LatticeData;
}

pub struct Lattice<const NDIM: usize, MarkerLattice: LatticeTypes>
where
    [(); 2 * NDIM - 2]:,
    [(); NDIM + 1]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
{
    edges: Node<EdgeConnections<NDIM>, <MarkerLattice as LatticeTypes>::EdgeData>,
}
