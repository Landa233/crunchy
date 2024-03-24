use crunchy::{
    lattice::{fields::Field, simulation::simulation::CubicalSimulation},
    math_utils::binomial_coefficient,
};

use super::lattice::{ZNLatticeTypes, ZNParameters};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MonopoleField<const NDIM: usize, const ZORDER: usize> {
    pub charge: isize,
}

impl<const NDIM: usize, const ZORDER: usize> Field for MonopoleField<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    type IndexType = [usize; NDIM + 1];
    type SimType = CubicalSimulation<NDIM, ZNLatticeTypes<NDIM, ZORDER>, ZNParameters<ZORDER>>;

    fn rebuild(node_index: Self::IndexType, grid: &mut Self::SimType) {
        let faces = grid.cubes[node_index].graph_connections.faces;

        let mut charge = 0;
        let mut alternator = 1;
        for face_index in faces.iter() {
            charge += alternator * grid.faces[*face_index].data.dirac_string;
            alternator *= -1;
        }

        grid.cubes[node_index].data.charge = charge;
    }
}
