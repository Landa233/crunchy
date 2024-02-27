use std::fmt::Debug;

use crate::{
    crarray::{self, crarray::CRArray, shape::Shape},
    math_utils::binomial_coefficient,
};

pub trait LatticeTypes {
    type VertexType: Field;
    type EdgeType: Field;
    type FaceType: Field;
    type CubeType: Field;
}

pub trait Field: Debug + Copy + Clone + Default {}

#[derive(Debug, Copy, Clone, Default)]
struct Node<GraphConnections: Default, FieldType: Field> {
    graph_connections: GraphConnections,
    data: FieldType,
}

fn my_default<const N: usize, const M: usize>() -> [[usize; N]; M] {
    [[0; N]; M]
}

#[derive(Debug, Copy, Clone)]
struct VertexConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
{
    edges: [[usize; NDIM + 1]; NDIM * 2],
    faces: [[usize; NDIM + 1]; 2 * (NDIM - 1)],
    cubes: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM, 2)],
}

impl<const NDIM: usize> Default for VertexConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
{
    fn default() -> Self {
        Self {
            edges: my_default(),
            faces: my_default(),
            cubes: my_default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct EdgeConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); 2 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
{
    vertices: [[usize; NDIM + 1]; 2],
    faces: [[usize; NDIM + 1]; 2 * binomial_coefficient(NDIM, 2)],
    cubes: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM - 1, 2)],
}

impl<const NDIM: usize> Default for EdgeConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); 2 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
{
    fn default() -> Self {
        Self {
            vertices: my_default(),
            faces: my_default(),
            cubes: my_default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct FaceConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 2)]:,
{
    vertices: [[usize; NDIM + 1]; 4],
    edges: [[usize; NDIM + 1]; 4],
    cubes: [[usize; NDIM + 1]; 2 * (NDIM - 2)],
}

impl<const NDIM: usize> Default for FaceConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 2)]:,
{
    fn default() -> Self {
        Self {
            vertices: my_default(),
            edges: my_default(),
            cubes: my_default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct CubeConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
{
    vertices: [[usize; NDIM + 1]; 8],
    edges: [[usize; NDIM + 1]; 12],
    faces: [[usize; NDIM + 1]; 6],
}

impl<const NDIM: usize> Default for CubeConnections<NDIM>
where
    [(); NDIM + 1]:,
{
    fn default() -> Self {
        Self {
            vertices: my_default(),
            edges: my_default(),
            faces: my_default(),
        }
    }
}

type VertexNode<const NDIM: usize, FieldType> = Node<VertexConnections<NDIM>, FieldType>;
type EdgeNode<const NDIM: usize, FieldType> = Node<EdgeConnections<NDIM>, FieldType>;
type FaceNode<const NDIM: usize, FieldType> = Node<FaceConnections<NDIM>, FieldType>;
type CubeNode<const NDIM: usize, FieldType> = Node<CubeConnections<NDIM>, FieldType>;

pub struct Lattice<const NDIM: usize, LTTypes: LatticeTypes>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 2 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
{
    shape: Shape<NDIM>,

    vertices: CRArray<{ NDIM + 1 }, VertexNode<NDIM, <LTTypes as LatticeTypes>::VertexType>>,
    edges: CRArray<{ NDIM + 1 }, EdgeNode<NDIM, <LTTypes as LatticeTypes>::EdgeType>>,
    faces: CRArray<{ NDIM + 1 }, FaceNode<NDIM, <LTTypes as LatticeTypes>::FaceType>>,
    cubes: CRArray<{ NDIM + 1 }, CubeNode<NDIM, <LTTypes as LatticeTypes>::CubeType>>,
}

impl<const NDIM: usize, LTTypes: LatticeTypes> Lattice<NDIM, LTTypes>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 2 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
{
    pub fn new(shape: Shape<NDIM>) -> Self {
        let vertices = CRArray::zeros(Shape::prepend(1, shape));
        let edges = CRArray::zeros(Shape::prepend(NDIM, shape));
        let faces = CRArray::zeros(Shape::prepend(binomial_coefficient(NDIM, 2), shape));
        let cubes = CRArray::zeros(Shape::prepend(binomial_coefficient(NDIM, 3), shape));

        println!("{:?}", binomial_coefficient(NDIM, 3));

        let mut res = Self {
            shape,
            vertices,
            edges,
            faces,
            cubes,
        };

        // for index in res.vertices.shape().iter() {
        //     println!("{:?}", res.vertices[index]);
        // }

        res
    }
}
