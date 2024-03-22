use crunchy::{
    lattice::{
        fields::{Field, UpdateField},
        simulation::simulation::CubicalSimulation,
    },
    math_utils::binomial_coefficient,
};

use super::{
    cubes::MonopoleField,
    lattice::{ZNLatticeTypes, ZNParameters},
    plaquettes::PlaquetteField,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeField<const NDIM: usize, const ZORDER: usize> {
    pub phase: usize,
}

impl<const NDIM: usize, const ZORDER: usize> Field for EdgeField<NDIM, ZORDER>
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
    // type SimType = Lattice<NDIM, ZNLatticeTypes<NDIM, ZORDER>>;
    type SimType = CubicalSimulation<NDIM, ZNLatticeTypes<NDIM, ZORDER>, ZNParameters<ZORDER>>;
}

impl<const NDIM: usize, const ZORDER: usize> UpdateField for EdgeField<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    fn update(node_index: Self::IndexType, grid: &mut Self::SimType, new_field: Self) -> Self {
        let old_field = grid.edges[node_index].data;
        grid.edges[node_index].data = new_field;

        let faces = grid.edges[node_index].graph_connections.faces;
        for plaquette_index in faces.iter() {
            PlaquetteField::<NDIM, ZORDER>::rebuild(*plaquette_index, grid);
        }

        let cubes = grid.edges[node_index].graph_connections.cubes;
        for cube_index in cubes.iter() {
            MonopoleField::<NDIM, ZORDER>::rebuild(*cube_index, grid);
        }

        old_field
    }

    fn energy(node_index: Self::IndexType, grid: &mut Self::SimType) -> f32 {
        let mut action = 0.;
        let ZNParameters {
            beta,
            cosines,
            lambda,
        } = grid.sim_parameters;

        for plaquette_id in grid.edges[node_index].graph_connections.faces.iter() {
            action += beta * (1. - 1. * cosines[grid.faces[*plaquette_id].data.holonomy])
        }

        for cube_id in grid.edges[node_index].graph_connections.cubes.iter() {
            action += lambda * grid.cubes[*cube_id].data.charge.abs() as f32;
        }

        action
    }

    fn init() -> impl FnMut(<Self as Field>::IndexType) -> Self
    where
        [(); NDIM + 1]:,
    {
        |_node_index: Self::IndexType| EdgeField::<NDIM, ZORDER> { phase: 0 }
    }
}
