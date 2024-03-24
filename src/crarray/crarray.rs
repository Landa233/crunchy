use core::fmt;
use std::{
    mem,
    ops::{Deref, Index, IndexMut},
};

use super::shape::{Shape, ShapeIterator};

#[derive(Debug, Clone)]
pub struct CRArray<const NDIM: usize, T> {
    shape: Shape<NDIM>,
    flat_data: Vec<T>,
}

impl<const NDIM: usize, T> CRArray<NDIM, T> {
    pub fn shape(&self) -> Shape<NDIM> {
        self.shape
    }

    pub fn zeros(shape: Shape<NDIM>) -> CRArray<NDIM, T>
    where
        T: Default + Clone,
    {
        return CRArray {
            shape,
            flat_data: vec![T::default(); shape.size()],
        };
    }

    pub fn iter(&self) -> ArrayIterator<'_, T, NDIM> {
        ArrayIterator {
            array: &self,
            shape_iterator: self.shape.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> ArrayIteratorMut<'_, T, NDIM> {
        ArrayIteratorMut {
            shape_iterator: self.shape.iter(),
            array: self,
        }
    }

    pub fn flat_data(&self) -> &[T] {
        &self.flat_data
    }
}

impl<const NDIM: usize, T> Index<[usize; NDIM]> for CRArray<NDIM, T> {
    type Output = T;

    fn index(&self, index: [usize; NDIM]) -> &Self::Output {
        &self.flat_data[self.shape.index_to_flat(index)]
    }
}

impl<const NDIM: usize, T> IndexMut<[usize; NDIM]> for CRArray<NDIM, T> {
    fn index_mut(&mut self, index: [usize; NDIM]) -> &mut Self::Output {
        &mut self.flat_data[self.shape.index_to_flat(index)]
    }
}

impl<const NDIM: usize, T> Deref for CRArray<NDIM, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.flat_data
    }
}

pub struct ArrayIterator<'a, T, const NDIM: usize> {
    array: &'a CRArray<NDIM, T>,
    shape_iterator: ShapeIterator<NDIM>,
}

impl<'a, T, const NDIM: usize> Iterator for ArrayIterator<'a, T, NDIM> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.shape_iterator.next();
        match index {
            Some(index) => Some(&self.array[index]),
            None => None,
        }
    }
}

pub struct ArrayIteratorMut<'a, T, const NDIM: usize> {
    array: &'a mut CRArray<NDIM, T>,
    shape_iterator: ShapeIterator<NDIM>,
}

impl<'a, T, const NDIM: usize> Iterator for ArrayIteratorMut<'a, T, NDIM> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.shape_iterator.next();

        match index {
            Some(index) => unsafe {
                let a: &mut T = mem::transmute(&mut self.array[index]);
                return Some(&mut *a);
            },
            None => None,
        }
    }
}

impl<const NDIM: usize, T: fmt::Display> fmt::Display for CRArray<NDIM, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for index in self.shape.iter() {
            writeln!(f, "------------------------------------------------------------------------------------------")?;
            writeln!(f, "Index: {:?}", index)?;
            write!(f, "{}", self[index])?;
        }
        Ok(())
    }
}
