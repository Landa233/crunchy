use rand::Rng;
use std::ops::Deref;

use rand::rngs::ThreadRng;

#[derive(Debug, Clone, Copy)]
pub struct Shape<const NDIM: usize> {
    pub dim: [usize; NDIM],
    pub ndim: usize,
}

impl<const NDIM: usize> Shape<NDIM> {
    pub fn new(dim: [usize; NDIM]) -> Self {
        for i in dim.into_iter() {
            if i == 0 {
                panic!(
                    "Can't create shapes with one of its dimensions 0.\nShape: {:?}",
                    dim
                )
            }
        }
        Self { dim, ndim: NDIM }
    }

    pub fn size(&self) -> usize {
        self.dim.iter().fold(1, |acc, &x| acc * x)
    }

    pub fn prepend(value: usize, shape: Self) -> Shape<{ NDIM + 1 }> {
        let mut res: Shape<{ NDIM + 1 }> = Shape::new([1; NDIM + 1]);
        res.dim[0] = value;

        for (axis, i) in shape.dim.into_iter().enumerate() {
            res.dim[axis + 1] = shape.dim[axis]
        }

        res
    }

    pub fn index_to_flat(&self, index: [usize; NDIM]) -> usize {
        let mut flat_index = 0;
        let mut stride = 1;

        for i in (0..self.ndim).rev() {
            if index[i] >= self.dim[i] {
                panic!("List index out of bounds for index {:?}", index);
            }
            flat_index += index[i] * stride;
            stride *= self.dim[i];
        }

        flat_index
    }

    pub fn flat_to_index(&self, mut flat_index: usize) -> [usize; NDIM] {
        let mut index = [0; NDIM];
        let mut stride = self.size();

        for i in 0..self.ndim {
            stride /= self.dim[i];
            index[i] = flat_index / stride;
            flat_index -= index[i] * stride;
        }

        index
    }

    pub fn iter(&self) -> ShapeIterator<NDIM> {
        ShapeIterator::new(*self)
    }

    pub fn random_index(&self, rng_gen: &mut ThreadRng) -> [usize; NDIM] {
        let size = self.size();
        let flat_index = rng_gen.gen_range(0..size);
        let index = self.flat_to_index(flat_index);

        index
    }
}

impl<const NDIM: usize> Deref for Shape<NDIM> {
    type Target = [usize; NDIM];

    fn deref(&self) -> &Self::Target {
        &self.dim
    }
}

impl<const NDIM: usize> From<[usize; NDIM]> for Shape<NDIM> {
    fn from(dim: [usize; NDIM]) -> Self {
        Self::new(dim)
    }
}

impl<const NDIM: usize> From<[u32; NDIM]> for Shape<NDIM> {
    fn from(dim: [u32; NDIM]) -> Self {
        let dim: [usize; NDIM] = dim
            .iter()
            .map(|&x| x as usize)
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        Self::new(dim)
    }
}

#[derive(Debug)]
pub struct ShapeIterator<const NDIM: usize> {
    shape: Shape<NDIM>,
    current_index: [usize; NDIM],
    finished: bool,
    started: bool,
}

impl<const NDIM: usize> ShapeIterator<NDIM> {
    pub fn new(shape: Shape<NDIM>) -> Self {
        Self {
            shape,
            current_index: [0; NDIM],
            finished: false,
            started: false,
        }
    }
}

impl<const NDIM: usize> Iterator for ShapeIterator<NDIM> {
    type Item = [usize; NDIM];

    fn next(&mut self) -> Option<Self::Item> {
        if self.started == false {
            self.started = true;
            return Some(self.current_index);
        }

        if self.finished == true {
            return None;
        }
        let mut current_dimension = self.shape.ndim - 1;
        let mut overflow = true;

        while overflow {
            self.current_index[current_dimension] += 1;
            if self.current_index[current_dimension] == self.shape.dim[current_dimension] {
                self.current_index[current_dimension] = 0;
                if current_dimension == 0 {
                    self.finished = true;
                    return None;
                } else {
                    current_dimension -= 1;
                }
            } else {
                overflow = false;
            }
        }
        Some(self.current_index)
    }
}
