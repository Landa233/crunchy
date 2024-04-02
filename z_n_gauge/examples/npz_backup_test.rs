use std::{fs::File, io, vec};

use byteorder::{LittleEndian, WriteBytesExt};
use npyz::{
    AutoSerialize, DType, Deserialize, NpyReader, Serialize, TypeRead, TypeWrite, WriterBuilder,
};
use rand_pcg::Pcg64Mcg;

#[derive(Debug)]
struct A {
    a: Vec<u8>,

    b: Vec<f32>,
}

impl Serialize for A {
    type TypeWriter = AWriter;

    fn writer(dtype: &DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(AWriter {})
    }
}

struct AWriter {}

impl TypeWrite for AWriter {
    type Value = A;

    fn write_one<W: io::Write>(&self, mut writer: W, value: &Self::Value) -> io::Result<()>
    where
        Self: Sized,
    {
        for &a in &value.a {
            writer.write_u8(a)?;
        }

        for &b in &value.b {
            writer.write_f32::<LittleEndian>(b)?;
        }

        Ok(())
    }
}

fn create_dtype(a: &A) -> DType {
    let dtype = DType::Record(vec![
        npyz::Field {
            name: "a".to_string(),
            dtype: DType::Array(
                a.a.len() as u64,
                Box::new(DType::Plain("<u1".parse().unwrap())),
            ),
        },
        npyz::Field {
            name: "b".to_string(),
            dtype: DType::Array(
                a.b.len() as u64,
                Box::new(DType::Plain("<f4".parse().unwrap())),
            ),
        },
    ]);

    dtype
}

impl Deserialize for A {
    type TypeReader = AReader;

    fn reader(dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(AReader {
            dtype: dtype.clone(),
        })
    }
}

struct AReader {
    dtype: DType,
}

impl TypeRead for AReader {
    type Value = A;

    fn read_one<R: io::Read>(&self, bytes: R) -> io::Result<Self::Value>
    where
        Self: Sized,
    {
        // How to read the bytes?

        let bytes = bytes.bytes().collect::<Result<Vec<u8>, _>>()?;

        let dtype = self.dtype.clone();

        let dtypes = match dtype {
            DType::Record(r) => {
                let mut dtypes = vec![];
                for field in r {
                    match field.name.as_str() {
                        "a" => {
                            let a = field.dtype;
                            dtypes.push(a);
                        }
                        "b" => {
                            let b = field.dtype;
                            dtypes.push(b);
                        }
                        _ => (),
                    }
                }
                dtypes
            }
            _ => panic!(),
        };

        let mut lengths = vec![];
        for dtype in dtypes {
            match dtype {
                DType::Array(len, _dtype) => lengths.push(len),
                _ => panic!(),
            }
        }

        let mut a = vec![];
        let mut b = vec![];

        let mut bytes = bytes.iter();

        // println!("{:?}", a);

        todo!()
    }
}

fn main() {
    let mut file = io::BufWriter::new(File::create("zzz.npy").unwrap());

    let a = A {
        a: vec![1, 2, 3, 5],
        b: vec![1.0, 2.0],
    };

    let mut writer = npyz::WriteOptions::new()
        .dtype(create_dtype(&a))
        // .shape(&[1])
        .writer(&mut file)
        .begin_1d()
        .unwrap();

    writer.push(&a).unwrap();
    writer.finish().unwrap();

    let bytes = std::fs::read("zzz.npy").unwrap();
    let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

    let vec: Vec<A> = reader.into_vec().unwrap();

    // let npy_data = NpyData

    // let dtype = npy.dtype();

    // let b: NpyReader<A, &[u8]> = npy.

    // for a in b {
    //     println!("{:?}", a.as_ref().unwrap().a);
    //     println!("{:?}", a.as_ref().unwrap().b);
    // }

    // let a: DType = npy.dtype();

    // println!("{:?}", a.descr());
    // match a {
    //     DType::Plain(_) => todo!(),
    //     DType::Array(_, _) => todo!(),
    //     DType::Record(r) => {
    //         for field in r {
    //             println!("{:?}", field.name);
    //             println!("{:?}", field.dtype);
    //         }
    //     }
    // }
}
