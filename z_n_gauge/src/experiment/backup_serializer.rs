use ndarray::{Array, IxDyn};
use npyz::{AutoSerialize, DType, Deserialize, Serialize, TypeRead, TypeWrite};
use rand_pcg::Pcg64Mcg;

use crate::sheduler::executor::ExecutorParameters;
use byteorder::{LittleEndian, WriteBytesExt};

use super::backup::LatticeBackup;

impl Serialize for LatticeBackup {
    type TypeWriter = BackupWriter;

    fn writer(dtype: &npyz::DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(BackupWriter {})
    }
}

pub fn array_dtype(array: &Array<u8, IxDyn>) -> DType {
    let shape = array.shape();

    let inner = Box::new(DType::Plain("<u1".parse().unwrap()));

    let mut dtype = inner;
    for dim in shape.iter().rev() {
        dtype = Box::new(DType::Array(*dim as u64, dtype))
    }

    return *dtype;
}

pub fn backup_dtype(backup: &LatticeBackup) -> DType {
    let rng_bytes = bincode::serialize(&backup.rng_gen).unwrap();

    let dtype = DType::Record(vec![
        npyz::Field {
            name: "edges_flat_data".to_string(),
            dtype: DType::Array(
                backup.edges_flat_data.len() as u64,
                Box::new(DType::Plain("<u1".parse().unwrap())),
            ),
        },
        npyz::Field {
            name: "rng_gen".to_string(),
            dtype: DType::Array(
                rng_bytes.len() as u64,
                Box::new(DType::Plain("<u1".parse().unwrap())),
            ),
        },
        npyz::Field {
            name: "performed_updates".to_string(),
            dtype: DType::Plain("<u8".parse().unwrap()),
        },
        npyz::Field {
            name: "accepted_updates".to_string(),
            dtype: DType::Plain("<u8".parse().unwrap()),
        },
        npyz::Field {
            name: "recordings".to_string(),
            dtype: array_dtype(&backup.recordings),
        },
        npyz::Field {
            name: "executor_parameters".to_string(),
            dtype: <ExecutorParameters as AutoSerialize>::default_dtype(),
        },
    ]);

    return dtype;
}

pub struct BackupWriter {}

impl TypeWrite for BackupWriter {
    type Value = LatticeBackup;

    fn write_one<W: std::io::Write>(
        &self,
        mut writer: W,
        value: &Self::Value,
    ) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let dtype = <ExecutorParameters as AutoSerialize>::default_dtype();
        let executor_writer = <ExecutorParameters as Serialize>::writer(&dtype).unwrap();

        for &edge in value.edges_flat_data.iter() {
            writer.write_u8(edge).unwrap();
        }

        // Serialize the rng
        let rng_bytes = bincode::serialize(&value.rng_gen).unwrap();
        for &byte in rng_bytes.iter() {
            writer.write_u8(byte).unwrap();
        }

        // Serialize performed_updates and accepted_updates
        writer
            .write_u64::<LittleEndian>(value.performed_updates)
            .unwrap();
        writer
            .write_u64::<LittleEndian>(value.accepted_updates)
            .unwrap();

        // Serialize recordings
        for &recording in value.recordings.iter() {
            writer.write_u8(recording).unwrap();
        }

        executor_writer.write_one(writer, &value.experiment_parameters)?;

        Ok(())
    }
}

impl Deserialize for LatticeBackup {
    type TypeReader = BackupReader;

    fn reader(dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(BackupReader {
            dtype: dtype.clone(),
        })
    }
}

pub struct BackupReader {
    dtype: DType,
}

impl TypeRead for BackupReader {
    type Value = LatticeBackup;

    fn read_one<R: std::io::Read>(&self, bytes: R) -> std::io::Result<Self::Value>
    where
        Self: Sized,
    {
        let bytes = bytes.bytes().collect::<Result<Vec<u8>, _>>().unwrap();

        let dtype = self.dtype.clone();

        let dtypes = match dtype {
            DType::Record(r) => {
                let mut dtypes = vec![];
                for field in r {
                    match field.name.as_str() {
                        "edges_flat_data" | "rng_gen" | "recordings" => {
                            dtypes.push((field.name.to_string(), field.dtype));
                        }

                        _ => (),
                    }
                }
                dtypes
            }
            _ => panic!(),
        };

        let mut shapes = vec![];

        fn shape_recursion(dtype: &DType, shape: &mut Vec<usize>) {
            match dtype {
                DType::Array(len, inner_dtype) => {
                    shape.push(*len as usize);
                    shape_recursion(inner_dtype, shape)
                }
                _ => (),
            }
        }
        for (name, dtype) in dtypes {
            let mut shape = vec![];
            shape_recursion(&dtype, &mut shape);
            shapes.push((name, shape));
        }
        fn shape_to_length(shape: &[usize]) -> usize {
            shape.iter().fold(1, |acc, &len| acc * len as usize)
        }

        let mut bytes_iter = bytes.iter();

        // Deserialize edges_flat_data
        let edges_flat_length = shape_to_length(&shapes[0].1);
        let edges_flat_data = bytes_iter
            .by_ref()
            .take(edges_flat_length)
            .map(|byte| *byte)
            .collect::<Vec<u8>>();

        // Deserialize rng_gen
        let rng_gen_length = shape_to_length(&shapes[1].1);
        let rng_gen_bytes = bytes_iter
            .by_ref()
            .take(rng_gen_length)
            .map(|byte| *byte)
            .collect::<Vec<u8>>();
        let rng_gen: Pcg64Mcg = bincode::deserialize(&rng_gen_bytes).unwrap();

        // Deserialize performed_updates and accepted_updates
        fn u8_to_u64(v: &mut std::slice::Iter<'_, u8>) -> u64 {
            let mut bytes = [0; 8];
            for (index, &byte) in v.take(8).enumerate() {
                bytes[index] = byte;
            }
            u64::from_le_bytes(bytes)
        }
        let performed_updates = u8_to_u64(&mut bytes_iter);
        let accepted_updates = u8_to_u64(&mut bytes_iter);

        // Deserialize Recordings
        let recordings_length = shape_to_length(&shapes[2].1);
        let recordings = bytes_iter
            .by_ref()
            .take(recordings_length)
            .map(|byte| *byte)
            .collect::<Vec<u8>>();

        let recordings_shape = shapes[2].1.clone();
        let recordings = Array::from_shape_vec(recordings_shape, recordings).unwrap();

        // Deserialize ExecutorParameters
        let rest_of_bytes = bytes_iter
            .into_iter()
            .map(|byte| *byte)
            .collect::<Vec<u8>>();

        let exec_par_reader = <ExecutorParameters as Deserialize>::reader(
            &<ExecutorParameters as AutoSerialize>::default_dtype(),
        )
        .unwrap();

        let executor_parameters = exec_par_reader.read_one(&rest_of_bytes[..]).unwrap();

        Ok(LatticeBackup {
            edges_flat_data,
            rng_gen,
            performed_updates,
            accepted_updates,
            recordings,
            experiment_parameters: executor_parameters,
        })
    }
}
