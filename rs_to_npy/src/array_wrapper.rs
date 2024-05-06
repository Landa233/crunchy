use std::{
    io,
    ops::{Deref, DerefMut},
};

use ndarray::{Array, IxDyn};
use npyz::{DType, Deserialize, Serialize, TypeRead, TypeWrite};

use crate::dtypeable::DTypeable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayWrapper<T> {
    pub data: Array<T, IxDyn>,
}

impl<T: DTypeable> DTypeable for ArrayWrapper<T> {
    fn generate_dtype(&self) -> DType {
        let inner_dtype = self.data.last().unwrap().generate_dtype();
        let shape = self.data.shape();

        for elem in self.data.iter() {
            if inner_dtype != elem.generate_dtype() {
                panic!(
                    "Unable to generate dtype for ArrayWrapper since elements have different dtypes:\n {:?}\n and\n {:?}",
                    inner_dtype,
                    elem.generate_dtype()
                );
            }
        }

        let mut res_dtype = inner_dtype;
        for dim in shape.iter().rev() {
            res_dtype = DType::Array(*dim as u64, Box::new(res_dtype));
        }

        res_dtype
    }
}

impl<T> Deref for ArrayWrapper<T> {
    type Target = Array<T, IxDyn>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ArrayWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Serialize + DTypeable> Serialize for ArrayWrapper<T> {
    type TypeWriter = ArrayWrapperWriter<T>;

    fn writer(_dtype: &DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(ArrayWrapperWriter {
            marker: std::marker::PhantomData,
        })
    }
}

pub struct ArrayWrapperWriter<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Serialize + DTypeable> TypeWrite for ArrayWrapperWriter<T> {
    type Value = ArrayWrapper<T>;

    fn write_one<W: io::Write>(&self, mut writer: W, value: &Self::Value) -> io::Result<()>
    where
        Self: Sized,
    {
        let inner_dtype = value.last().unwrap().generate_dtype();
        let type_writer = <T as Serialize>::writer(&inner_dtype).unwrap();

        for e in value.iter() {
            type_writer.write_one(&mut writer, e)?;
        }
        Ok(())
    }
}

impl<T: Deserialize> Deserialize for ArrayWrapper<T> {
    type TypeReader = ArrayWrapperReader<T>;

    fn reader(dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(ArrayWrapperReader {
            marker: std::marker::PhantomData,
            dtype: dtype.clone(),
        })
    }
}

pub struct ArrayWrapperReader<T> {
    marker: std::marker::PhantomData<T>,
    dtype: DType,
}

impl<T: Deserialize> TypeRead for ArrayWrapperReader<T> {
    type Value = ArrayWrapper<T>;

    fn read_one<R: io::Read>(&self, mut b: R) -> io::Result<Self::Value>
    where
        Self: Sized,
    {
        let mut shape = vec![];
        let mut dtype = &self.dtype;

        while let DType::Array(dim, inner_dtype) = dtype {
            shape.push(*dim as usize);
            dtype = inner_dtype;
        }

        let most_inner_dtype = dtype;
        let type_reader = <T as Deserialize>::reader(most_inner_dtype).unwrap();

        let ndim = shape.iter().fold(1, |acc, x| acc * x);

        let mut array_data = vec![];
        for _ in 0..ndim {
            let a = type_reader.read_one(&mut b)?;
            array_data.push(a);
        }

        Ok(ArrayWrapper {
            data: Array::from_shape_vec(shape, array_data).unwrap(),
        })
    }
}
