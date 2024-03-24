use std::fmt;

use crate::math_utils::{binomial_coefficient, my_default};

#[derive(Debug, Copy, Clone)]
pub struct VertexConnections<const NDIM: usize>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    pub edges: [[usize; NDIM + 1]; NDIM * 2],
    pub faces: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM, 2)],
    pub cubes: [[usize; NDIM + 1]; 8 * binomial_coefficient(NDIM, 3)],
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
    pub vertices: [[usize; NDIM + 1]; 2],
    pub faces: [[usize; NDIM + 1]; 2 * (NDIM - 1)],
    pub cubes: [[usize; NDIM + 1]; 4 * binomial_coefficient(NDIM - 1, 2)],
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
    pub vertices: [[usize; NDIM + 1]; 4],
    pub edges: [[usize; NDIM + 1]; 4],
    pub cubes: [[usize; NDIM + 1]; 2 * (NDIM - 2)],
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
    pub vertices: [[usize; NDIM + 1]; 8],
    pub edges: [[usize; NDIM + 1]; 12],
    pub faces: [[usize; NDIM + 1]; 6],
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
