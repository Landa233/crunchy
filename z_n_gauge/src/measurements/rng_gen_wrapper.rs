use std::ops::Deref;

use byteorder::WriteBytesExt;
use npyz::{DType, Deserialize, Field, Serialize};
use rand_pcg::Pcg64Mcg;
use rs_to_npy::dtypeable::DTypeable;

#[derive(Debug, Clone, PartialEq)]
pub struct RngGenWrapper {
    pub rng_gen: Pcg64Mcg,
}

impl Serialize for RngGenWrapper {
    type TypeWriter = RngGenWrapperWriter;

    fn writer(_dtype: &npyz::DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(RngGenWrapperWriter {})
    }
}

pub struct RngGenWrapperWriter {}

impl npyz::TypeWrite for RngGenWrapperWriter {
    type Value = RngGenWrapper;

    fn write_one<W: std::io::Write>(
        &self,
        mut writer: W,
        value: &Self::Value,
    ) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let rng_bytes = bincode::serialize(&value.rng_gen).unwrap();
        for &byte in rng_bytes.iter() {
            writer.write_u8(byte).unwrap();
        }

        Ok(())
    }
}

impl DTypeable for RngGenWrapper {
    fn generate_dtype(&self) -> npyz::DType {
        let rng_bytes = bincode::serialize(&self.rng_gen).unwrap();
        DType::Record(vec![Field {
            name: "rng_gen_wrapper".to_string(),
            dtype: DType::Array(
                rng_bytes.len() as u64,
                Box::new(DType::Plain("<u1".parse().unwrap())),
            ),
        }])
    }
}

impl Deserialize for RngGenWrapper {
    type TypeReader = RngGenWrapperReader;

    fn reader(_dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(RngGenWrapperReader {})
    }
}

pub struct RngGenWrapperReader {}

impl npyz::TypeRead for RngGenWrapperReader {
    type Value = RngGenWrapper;

    fn read_one<R: std::io::Read>(&self, mut reader: R) -> std::io::Result<Self::Value> {
        // Read 16 bytes from reader
        let mut rng_bytes = vec![0; 16];
        reader.read_exact(&mut rng_bytes).unwrap();

        let rng_gen: Pcg64Mcg = bincode::deserialize(&rng_bytes).unwrap();

        Ok(RngGenWrapper { rng_gen })
    }
}

impl Deref for RngGenWrapper {
    type Target = Pcg64Mcg;

    fn deref(&self) -> &Self::Target {
        &self.rng_gen
    }
}
