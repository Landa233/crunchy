use core::fmt;
use std::{fmt::Debug, marker::PhantomData, usize};

use rand::Rng;

use crate::{
    crarray::{self, crarray::CRArray, shape::Shape},
    lattice::ind::Ind,
    math_utils::binomial_coefficient,
};

pub trait SimParameter: Copy + Clone {}

pub trait LatticeTypes {
    const NDIM: usize;

    type VertexType: Field;
    type EdgeType: Field;
    type FaceType: Field;
    type CubeType: Field;

    type SimParameterType: SimParameter;
}

pub trait Field: Debug + Copy + Clone + Default {
    type LatticeMarker: LatticeTypes;

    fn rebuild(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM * 2]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 1)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 2)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM - 1, 2)]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 2)]:,
        [(); 8 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 3)]:,
    {
    }
}

pub trait UpdateField: Field {
    fn update(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
        new_field: Self,
    ) -> Self
    where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM * 2]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 1)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 2)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM - 1, 2)]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 2)]:,
        [(); 8 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 3)]:;

    // fn set(
    //     node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
    //     grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    //     field: Self,
    // ) where
    //     [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
    //     [(); <Self::LatticeMarker as LatticeTypes>::NDIM * 2]:,
    //     [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 1)]:,
    //     [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 2)]:,
    //     [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM - 1, 2)]:,
    //     [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 2)]:,
    //     [(); 8 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 3)]:;

    fn energy(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
    ) -> f32
    where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM * 2]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 1)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 2)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM - 1, 2)]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 2)]:,
        [(); 8 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 3)]:;

    fn init() -> impl FnMut([usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1]) -> Self
    where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:;

    fn metropolis_step(
        node_index: [usize; <Self::LatticeMarker as LatticeTypes>::NDIM + 1],
        grid: &mut Lattice<{ <Self::LatticeMarker as LatticeTypes>::NDIM }, Self::LatticeMarker>,
        new_field: Self,
        rng_gen: &mut rand::rngs::ThreadRng,
    ) where
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM + 1]:,
        [(); <Self::LatticeMarker as LatticeTypes>::NDIM * 2]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 1)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 2)]:,
        [(); 4 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM - 1, 2)]:,
        [(); 2 * (<Self::LatticeMarker as LatticeTypes>::NDIM - 2)]:,
        [(); 8 * binomial_coefficient(<Self::LatticeMarker as LatticeTypes>::NDIM, 3)]:,
    {
        let old_energy = Self::energy(node_index, grid);

        // Self::set(node_index, grid, new_field);
        let old_field = Self::update(node_index, grid, new_field);
        let new_energy = Self::energy(node_index, grid);

        let delta_energy = new_energy - old_energy;

        if delta_energy < 0.0 {
            return;
        }

        let acceptance_probability = (-delta_energy).exp();

        let random_number: f32 = rng_gen.gen_range(0.0..1.0); // random number between 0 and 1

        if random_number < acceptance_probability {
            return;
        }

        // Self::set(node_index, grid, old_field);
        Self::update(node_index, grid, old_field);
    }
}

// pub trait Initializer<FieldType: Field> {
//     fn init(
//         &mut self,
//         index: [usize; <<FieldType as Field>::LatticeMarker as LatticeTypes>::NDIM + 1],
//     ) -> FieldType;
// }

// impl<FieldType: Field, F> Initializer<FieldType> for F
// where
//     F: FnMut([usize; <<FieldType as Field>::LatticeMarker as LatticeTypes>::NDIM + 1]) -> FieldType,
// {
//     fn init(
//         &mut self,
//         index: [usize; <<FieldType as Field>::LatticeMarker as LatticeTypes>::NDIM + 1],
//     ) -> FieldType {
//         self(index)
//     }
// }

#[derive(Debug, Copy, Clone, Default)]
pub struct Node<GraphConnections: Default, FieldType: Field> {
    pub graph_connections: GraphConnections,
    pub data: FieldType,
}

impl<GraphConnections: Default + fmt::Display, FieldType: Field + fmt::Display> fmt::Display
    for Node<GraphConnections, FieldType>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\nConnections: \n{}\n\nData:\n{}\n",
            self.graph_connections, self.data
        )?;

        Ok(())
    }
}

fn my_default<const N: usize, const M: usize>() -> [[usize; N]; M] {
    [[0; N]; M]
}

#[derive(Debug, Copy, Clone)]
pub struct VertexConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    edges: [[usize; NDIM + 1]; NDIM * 2],
    faces: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM, 2)],
    cubes: [[usize; NDIM + 1]; 8 * binomial_coefficient(NDIM, 3)],
}

impl<const NDIM: usize> Default for VertexConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    fn default() -> Self {
        Self {
            edges: my_default(),
            faces: my_default(),
            cubes: my_default(),
        }
    }
}

impl<const NDIM: usize> fmt::Display for VertexConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Edges: {:?}", self.edges)?;
        writeln!(f, "Faces: {:?}", self.faces)?;
        write!(f, "Cubes: {:?}", self.cubes)?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EdgeConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
{
    vertices: [[usize; NDIM + 1]; 2],
    faces: [[usize; NDIM + 1]; 2 * (NDIM - 1)],
    cubes: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM - 1, 2)],
}

impl<const NDIM: usize> Default for EdgeConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 1)]:,
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

impl<const NDIM: usize> fmt::Display for EdgeConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vertices: {:?}", self.vertices)?;
        writeln!(f, "Faces: {:?}", self.faces)?;
        write!(f, "Cubes: {:?}", self.cubes)?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FaceConnections<const NDIM: usize>
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

impl<const NDIM: usize> fmt::Display for FaceConnections<NDIM>
where
    [(); NDIM + 1]:,
    [(); 2 * (NDIM - 2)]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vertices: {:?}", self.vertices)?;
        writeln!(f, "Edges: {:?}", self.edges)?;
        write!(f, "Cubes: {:?}", self.cubes)?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CubeConnections<const NDIM: usize>
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

impl<const NDIM: usize> fmt::Display for CubeConnections<NDIM>
where
    [(); NDIM + 1]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vertices: {:?}", self.vertices)?;
        writeln!(f, "Edges: {:?}", self.edges)?;
        write!(f, "Faces: {:?}", self.faces)?;

        Ok(())
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
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    shape: Shape<NDIM>,

    pub vertices: CRArray<{ NDIM + 1 }, VertexNode<NDIM, <LTTypes as LatticeTypes>::VertexType>>,
    pub edges: CRArray<{ NDIM + 1 }, EdgeNode<NDIM, <LTTypes as LatticeTypes>::EdgeType>>,
    pub faces: CRArray<{ NDIM + 1 }, FaceNode<NDIM, <LTTypes as LatticeTypes>::FaceType>>,
    pub cubes: CRArray<{ NDIM + 1 }, CubeNode<NDIM, <LTTypes as LatticeTypes>::CubeType>>,

    pub sim_parameters: LTTypes::SimParameterType,
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
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    pub fn new(shape: Shape<NDIM>, sim_parameters: LTTypes::SimParameterType) -> Self {
        let vertices: CRArray<
            _,
            Node<VertexConnections<NDIM>, <LTTypes as LatticeTypes>::VertexType>,
        > = CRArray::zeros(Shape::prepend(1, shape));
        let edges = CRArray::zeros(Shape::prepend(NDIM, shape));
        let faces = CRArray::zeros(Shape::prepend(binomial_coefficient(NDIM, 2), shape));
        let cubes = CRArray::zeros(Shape::prepend(binomial_coefficient(NDIM, 3), shape));

        let mut res = Self {
            shape,
            vertices,
            edges,
            faces,
            cubes,
            sim_parameters,
        };

        if NDIM > 2 {
            res.initialize_cubes();
        }
        res.initialize_faces();
        res.initialize_edges();

        res
    }

    fn standard_basis() -> [(usize, [usize; NDIM]); NDIM] {
        let mut res = [(0, [0; NDIM]); NDIM];
        for i in 0..NDIM {
            res[i].0 = i;
            res[i].1[i] = 1;
        }
        res
    }

    fn cube_planes() -> [[usize; 3]; binomial_coefficient(NDIM, 3)] {
        let mut res = [[0; 3]; binomial_coefficient(NDIM, 3)];

        let mut counter = 0;
        for i in 0..NDIM {
            for j in (i + 1)..NDIM {
                for k in (j + 1)..NDIM {
                    res[counter] = [i, j, k];
                    counter += 1;
                }
            }
        }

        res
    }

    pub fn initialize_cubes(&mut self) {
        let cube_shape = self.cubes.shape();

        let standard_basis = Self::standard_basis();

        let cube_planes = Self::cube_planes();

        let grid_dim = self.shape.dim;

        let mut cubes_to_faces: CRArray<{ NDIM + 1 }, Vec<[usize; NDIM + 1]>> =
            CRArray::zeros(self.faces.shape());

        // --------------------------------------------------------
        // CUBES <-> FACES
        // --------------------------------------------------------
        for index in cube_shape.iter() {
            let cube_dirs = cube_planes[index[0]];
            let e_1 = standard_basis[cube_dirs[0]];
            let e_2 = standard_basis[cube_dirs[1]];
            let e_3 = standard_basis[cube_dirs[2]];

            let anchor: Ind<NDIM> = Ind::new(index[1..].try_into().unwrap());

            let mut faces: Vec<[usize; NDIM + 1]> = vec![];

            // This is where the periodic boundary conditions are implemented
            faces.push(
                *(((anchor + e_1.1) % grid_dim).prepend(Self::plaquette_plane(e_2.0, e_3.0))),
            );
            faces.push(*((anchor).prepend(Self::plaquette_plane(e_2.0, e_3.0))));

            // SWITCHED THE ORDER HERE BECAUSE OF ORIENTATIONS SO THAT FOR THE DIFFERENTIAL
            // WE CAN JUST TAKE THE ALTERNATING SUM OF THE FACES
            faces.push(*((anchor).prepend(Self::plaquette_plane(e_1.0, e_3.0))));
            faces.push(
                *(((anchor + e_2.1) % grid_dim).prepend(Self::plaquette_plane(e_1.0, e_3.0))),
            );

            faces.push(
                *(((anchor + e_3.1) % grid_dim).prepend(Self::plaquette_plane(e_1.0, e_2.0))),
            );
            faces.push(*((anchor).prepend(Self::plaquette_plane(e_1.0, e_2.0))));

            for face in faces.iter() {
                cubes_to_faces[*face].push(index);
            }

            self.cubes[index].graph_connections.faces = faces.try_into().unwrap();
        }

        for index in self.faces.shape().iter() {
            self.faces[index].graph_connections.cubes =
                cubes_to_faces[index].clone().try_into().unwrap();
        }

        // --------------------------------------------------------
        // CUBES <-> EDGES
        // --------------------------------------------------------

        let mut cube_to_edges: CRArray<{ NDIM + 1 }, Vec<[usize; NDIM + 1]>> =
            CRArray::zeros(self.edges.shape());

        for cube_index in cube_shape.iter() {
            let mut edges = vec![];

            let cube_dirs = cube_planes[cube_index[0]];
            let e_1 = standard_basis[cube_dirs[0]];
            let e_2 = standard_basis[cube_dirs[1]];
            let e_3 = standard_basis[cube_dirs[2]];

            let anchor: Ind<NDIM> = Ind::new(cube_index[1..].try_into().unwrap());

            for [v1, v2, v3] in [[e_1, e_2, e_3], [e_2, e_3, e_1], [e_3, e_1, e_2]] {
                edges.push(*(anchor.prepend(v1.0)));
                edges.push(*(((anchor + v2.1) % grid_dim).prepend(v1.0)));
                edges.push(*(((anchor + v3.1) % grid_dim).prepend(v1.0)));
                edges.push(*(((anchor + v2.1 + v3.1) % grid_dim).prepend(v1.0)));
            }

            for edge in edges.iter() {
                cube_to_edges[*edge].push(cube_index);
            }

            self.cubes[cube_index].graph_connections.edges = edges.try_into().unwrap();
        }

        for index in self.edges.shape().iter() {
            self.edges[index].graph_connections.cubes =
                cube_to_edges[index].clone().try_into().unwrap();
        }

        // --------------------------------------------------------
        // CUBES <-> VERTICES
        // --------------------------------------------------------
        let mut cube_to_vertices: CRArray<{ NDIM + 1 }, Vec<[usize; NDIM + 1]>> =
            CRArray::zeros(self.vertices.shape());

        for cube_index in cube_shape.iter() {
            let mut vertices = vec![];

            let cube_dirs = cube_planes[cube_index[0]];
            let e_1 = standard_basis[cube_dirs[0]];
            let e_2 = standard_basis[cube_dirs[1]];
            let e_3 = standard_basis[cube_dirs[2]];

            let anchor: Ind<NDIM> = Ind::new(cube_index[1..].try_into().unwrap());

            vertices.push(*anchor.prepend(0));
            vertices.push(*((anchor + e_1.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_2.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_3.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_1.1 + e_2.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_1.1 + e_3.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_2.1 + e_3.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_1.1 + e_2.1 + e_3.1) % grid_dim).prepend(0));

            for vertex in vertices.iter() {
                cube_to_vertices[*vertex].push(cube_index);
            }

            self.cubes[cube_index].graph_connections.vertices = vertices.try_into().unwrap();
        }

        for index in self.vertices.shape().iter() {
            self.vertices[index].graph_connections.cubes =
                cube_to_vertices[index].clone().try_into().unwrap();
        }
    }

    fn initialize_faces(&mut self) {
        let face_shape = self.faces.shape();

        let standard_basis = Self::standard_basis();

        let face_planes = Self::face_planes();

        let grid_dim = self.shape.dim;

        // --------------------------------------------------------
        // FACES <-> EDGES
        // --------------------------------------------------------
        let mut faces_to_edges: CRArray<{ NDIM + 1 }, Vec<[usize; NDIM + 1]>> =
            CRArray::zeros(self.edges.shape());

        for index in face_shape.iter() {
            let face_dirs = face_planes[index[0]];
            let e_1 = standard_basis[face_dirs[0]];
            let e_2 = standard_basis[face_dirs[1]];

            let anchor: Ind<NDIM> = Ind::new(index[1..].try_into().unwrap());

            let mut edges: Vec<[usize; NDIM + 1]> = vec![];

            edges.push(*((anchor + e_1.1) % grid_dim).prepend(e_2.0));
            edges.push(*((anchor).prepend(e_2.0)));

            edges.push(*((anchor).prepend(e_1.0)));
            edges.push(*((anchor + e_2.1) % grid_dim).prepend(e_1.0));

            for edge in edges.iter() {
                faces_to_edges[*edge].push(index);
            }

            self.faces[index].graph_connections.edges = edges.try_into().unwrap();
        }

        for index in self.edges.shape().iter() {
            self.edges[index].graph_connections.faces =
                faces_to_edges[index].clone().try_into().unwrap();
        }

        // --------------------------------------------------------
        // FACES <-> VERTICES
        // --------------------------------------------------------
        let mut faces_to_vertices: CRArray<{ NDIM + 1 }, Vec<[usize; NDIM + 1]>> =
            CRArray::zeros(self.vertices.shape());

        for index in face_shape.iter() {
            let face_dirs = face_planes[index[0]];
            let e_1 = standard_basis[face_dirs[0]];
            let e_2 = standard_basis[face_dirs[1]];

            let anchor: Ind<NDIM> = Ind::new(index[1..].try_into().unwrap());

            let mut vertices: Vec<[usize; NDIM + 1]> = vec![];

            vertices.push(*anchor.prepend(0));
            vertices.push(*((anchor + e_1.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_1.1 + e_2.1) % grid_dim).prepend(0));
            vertices.push(*((anchor + e_2.1) % grid_dim).prepend(0));

            for vertex in vertices.iter() {
                faces_to_vertices[*vertex].push(index);
            }

            self.faces[index].graph_connections.vertices = vertices.try_into().unwrap();
        }

        for index in self.vertices.shape().iter() {
            self.vertices[index].graph_connections.faces =
                faces_to_vertices[index].clone().try_into().unwrap();
        }
    }

    fn initialize_edges(&mut self) {
        let edge_shape = self.edges.shape();

        let standard_basis = Self::standard_basis();

        let grid_dim = self.shape.dim;

        // --------------------------------------------------------
        // EDGES <-> VERTICES
        // --------------------------------------------------------
        let mut edges_to_vertices: CRArray<{ NDIM + 1 }, Vec<[usize; NDIM + 1]>> =
            CRArray::zeros(self.vertices.shape());

        for index in edge_shape.iter() {
            let anchor: Ind<NDIM> = Ind::new(index[1..].try_into().unwrap());

            let mut vertices: Vec<[usize; NDIM + 1]> = vec![];

            let v = standard_basis[index[0]].1;

            vertices.push(*anchor.prepend(0));
            vertices.push(*((anchor + v) % grid_dim).prepend(0));

            for vertex in vertices.iter() {
                edges_to_vertices[*vertex].push(index);
            }

            self.edges[index].graph_connections.vertices = vertices.try_into().unwrap();
        }

        for index in self.vertices.shape().iter() {
            self.vertices[index].graph_connections.edges =
                edges_to_vertices[index].clone().try_into().unwrap();
        }
    }

    fn face_planes() -> [[usize; 2]; binomial_coefficient(NDIM, 2)] {
        let mut res = [[0; 2]; binomial_coefficient(NDIM, 2)];

        let mut counter = 0;
        for i in 0..NDIM {
            for j in (i + 1)..NDIM {
                res[counter] = [i, j];
                counter += 1;
            }
        }

        res
    }

    fn plaquette_plane(i: usize, j: usize) -> usize {
        if i >= j {
            panic!("The set ({}, {}) is not ordered", i, j)
        }

        debug_assert!(
            i < NDIM && j < NDIM,
            " Either i: {i} or j: {j} are bigger than {NDIM}"
        );

        let mut a: usize = 0;
        for k in 1..(i + 1) {
            a += NDIM - k
        }

        return a + (j - i) - 1;
    }

    fn cube_directions() -> [[usize; 3]; binomial_coefficient(NDIM, 3)] {
        let mut cube_directions = [[0; 3]; binomial_coefficient(NDIM, 3)];
        let mut counter = 0;
        for i in 0..NDIM {
            for j in (i + 1)..NDIM {
                for k in (j + 1)..NDIM {
                    cube_directions[counter] = [i, j, k];
                    counter += 1;
                }
            }
        }

        cube_directions
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct EmptyField<AnyL> {
    marker: PhantomData<AnyL>,
}

impl<AnyL: LatticeTypes + Debug + Copy + Clone + Default> Field for EmptyField<AnyL> {
    type LatticeMarker = AnyL;
}
