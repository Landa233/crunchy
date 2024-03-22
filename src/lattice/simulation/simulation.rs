use std::ops::{Deref, DerefMut};

use crate::{crarray::shape::Shape, lattice::cubical_lattice::CubicalLattice};

pub struct Simulation<const NDIM: usize, SimParameterType, LatticeType> {
    pub lattice: LatticeType,

    pub sim_parameters: SimParameterType,
}

pub type CubicalSimulation<const NDIM: usize, CubicalFields, SimParameterType> =
    Simulation<NDIM, SimParameterType, CubicalLattice<NDIM, CubicalFields>>;

// Derefs to Lattice, so that we can access the lattice fields directly via methods.
// Testing how this feels.
impl<const NDIM: usize, SimParameterType, LatticeType> Deref
    for Simulation<NDIM, SimParameterType, LatticeType>
{
    type Target = LatticeType;

    fn deref(&self) -> &Self::Target {
        &self.lattice
    }
}

impl<const NDIM: usize, SimParameterType, LatticeType> DerefMut
    for Simulation<NDIM, SimParameterType, LatticeType>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lattice
    }
}
