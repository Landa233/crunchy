use crate::gauge_fields::lattice::ZNLatticeTypes;
use crate::gauge_fields::lattice::ZNParameters;
use crunchy::lattice::fields::Field;
use crunchy::math_utils::binomial_coefficient;
use crunchy::simulation::simulation::CubicalSimulation;
use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlaquetteField<const NDIM: usize, const ZORDER: usize> {
    pub holonomy: usize,
    pub dirac_string: isize,
}

impl<const NDIM: usize, const ZORDER: usize> fmt::Display for PlaquetteField<NDIM, ZORDER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlaquetteField: {}", self.holonomy)
    }
}

impl<const NDIM: usize, const ZORDER: usize> Field for PlaquetteField<NDIM, ZORDER>
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
        let edges = grid.faces[node_index].graph_connections.edges;

        let mut holonomy = 2 * ZORDER;
        holonomy += grid.edges[edges[0]].data.phase;
        holonomy -= grid.edges[edges[1]].data.phase;
        holonomy += grid.edges[edges[2]].data.phase;
        holonomy -= grid.edges[edges[3]].data.phase;

        grid.faces[node_index].data.holonomy = holonomy % ZORDER;

        let dirac_string = calculate_dirac_string::<NDIM, ZORDER>(node_index, grid);
        grid.faces[node_index].data.dirac_string = dirac_string;
    }
}

fn calculate_dirac_string<const NDIM: usize, const ZORDER: usize>(
    face_index: [usize; NDIM + 1],
    grid: &CubicalSimulation<NDIM, ZNLatticeTypes<NDIM, ZORDER>, ZNParameters<ZORDER>>,
) -> isize
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    fn shift<const ZORDER: usize>(phase: usize) -> isize {
        if phase <= ZORDER / 2 {
            return phase as isize;
        }
        return phase as isize - ZORDER as isize;
    }

    let mut windings = 0;
    let edges = grid.faces[face_index].graph_connections.edges;
    windings += shift::<ZORDER>(grid.edges[edges[0]].data.phase);
    windings -= shift::<ZORDER>(grid.edges[edges[1]].data.phase);
    windings += shift::<ZORDER>(grid.edges[edges[2]].data.phase);
    windings -= shift::<ZORDER>(grid.edges[edges[3]].data.phase);

    if ZORDER != 3 {
        panic!("Only implemented for Z3")
    }

    let mut res = 0;
    // For ZORDER > 3, windings of 2?
    if windings < -(ZORDER as isize) / 2 {
        res = -1;
    }
    if windings > ZORDER as isize / 2 {
        res = 1;
    }

    res
}
