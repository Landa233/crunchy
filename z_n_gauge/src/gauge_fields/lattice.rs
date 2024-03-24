use crunchy::{
    lattice::{cubical_lattice::CubicalFields, fields::EmptyField},
    math_utils::binomial_coefficient,
    simulation::simulation::SimParameter,
};

use super::{cubes::MonopoleField, edges::EdgeField, plaquettes::PlaquetteField};

#[derive(Debug, Clone, Copy, Default)]
pub struct ZNLatticeTypes<const NDIM: usize, const ZORDER: usize> {}

impl<const NDIM: usize, const ZORDER: usize> CubicalFields for ZNLatticeTypes<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    const NDIM: usize = NDIM;

    type VertexField = EmptyField<ZNLatticeTypes<NDIM, ZORDER>>;
    type EdgeField = EdgeField<NDIM, ZORDER>;
    type FaceField = PlaquetteField<NDIM, ZORDER>;
    type CubeField = MonopoleField<NDIM, ZORDER>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZNParameters<const ZORDER: usize> {
    pub beta: f32,
    pub cosines: [f32; ZORDER],
    pub lambda: f32,
}

impl<const ZORDER: usize> SimParameter for ZNParameters<ZORDER> {}
