use core::fmt;
use std::{fmt::Debug, marker::PhantomData, usize};

use rand::Rng;

use crate::{
    crarray::{self, crarray::CRArray, ind::Ind, shape::Shape},
    math_utils::binomial_coefficient,
};

pub trait SimParameter: Copy + Clone {}

pub trait Field: Debug + Copy + Clone + Default {
    type IndexType: Copy + Clone;
    type SimType;

    fn rebuild(node_index: Self::IndexType, grid: &mut Self::SimType) {}
}

pub trait UpdateField: Field {
    fn update(node_index: Self::IndexType, grid: &mut Self::SimType, new_field: Self) -> Self;

    fn energy(node_index: Self::IndexType, grid: &mut Self::SimType) -> f32;

    fn init() -> impl FnMut(Self::IndexType) -> Self;

    fn metropolis_step<R: Rng>(
        node_index: Self::IndexType,
        grid: &mut Self::SimType,
        new_field: Self,
        rng_gen: &mut R,
    ) {
        let old_energy = Self::energy(node_index, grid);
        let old_field = Self::update(node_index, grid, new_field);
        let new_energy = Self::energy(node_index, grid);

        let delta_energy = new_energy - old_energy;
        if delta_energy < 0.0 {
            return;
        }

        let acceptance_probability = (-delta_energy).exp();
        let random_number: f32 = rng_gen.gen_range(0.0..1.0);
        if random_number < acceptance_probability {
            return;
        }

        Self::update(node_index, grid, old_field);
    }
}

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

#[derive(Debug, Copy, Clone, Default)]
pub struct EmptyField<AnyL> {
    marker: PhantomData<AnyL>,
}

impl<AnyL: Debug + Copy + Clone + Default> Field for EmptyField<AnyL> {
    type IndexType = ();
    type SimType = AnyL;
}
