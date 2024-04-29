use ndarray::Array;
use npyz::{DType, Deserialize, TypeRead};
use rand_pcg::Pcg64Mcg;

use crate::experiment::backup::backup::{ExperimentParameters, LastState, RunInfo};

use super::backup::{BackUp, BackupData, RebootSeed};

pub struct BackupReader {
    dtype: DType,
}

impl Deserialize for BackUp {
    type TypeReader = BackupReader;

    fn reader(dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(BackupReader {
            dtype: dtype.clone(),
        })
    }
}

impl TypeRead for BackupReader {
    type Value = BackUp;

    fn read_one<R: std::io::Read>(&self, bytes: R) -> std::io::Result<Self::Value>
    where
        Self: Sized,
    {
        let bytes = bytes.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
        let dtype = self.dtype.clone();

        let mut dtypes = match dtype {
            DType::Record(r) => {
                let mut dtypes = vec![];
                for field in r {
                    match field.name.as_str() {
                        "reboot_seed" | "backup_data" | "run_info" => {
                            dtypes.push(field.dtype);
                        }

                        _ => (),
                    }
                }
                dtypes
            }
            _ => panic!(),
        };

        let reboot_seed_dtype = dtypes.remove(0);
        let backup_data_dtype = dtypes.remove(0);
        let run_info_dtype = dtypes.remove(0);

        let rseed_byte_num = reboot_seed_dtype.num_bytes().unwrap();
        let reboot_seed_reader = <RebootSeed as Deserialize>::reader(&reboot_seed_dtype).unwrap();
        let reboot_seed = reboot_seed_reader
            .read_one(&bytes[..rseed_byte_num])
            .unwrap();

        let backup_data_byte_num = backup_data_dtype.num_bytes().unwrap();
        let backup_data_reader = <BackupData as Deserialize>::reader(&backup_data_dtype).unwrap();
        let backup_data = backup_data_reader
            .read_one(&bytes[rseed_byte_num..(backup_data_byte_num + rseed_byte_num)])
            .unwrap();

        let run_info_reader = <RunInfo as Deserialize>::reader(&run_info_dtype).unwrap();
        let run_info = run_info_reader
            .read_one(&bytes[(backup_data_byte_num + rseed_byte_num)..])
            .unwrap();

        let res = BackUp {
            reboot_seed,
            backup_data,
            run_info,
        };

        Ok(res)
    }
}

fn shape_recursion(dtype: &DType, shape: &mut Vec<usize>) {
    match dtype {
        DType::Array(len, inner_dtype) => {
            shape.push(*len as usize);
            shape_recursion(inner_dtype, shape)
        }
        _ => (),
    }
}

fn shape_to_length(shape: &[usize]) -> usize {
    shape.iter().fold(1, |acc, &len| acc * len as usize)
}

pub struct RebootSeedReader {
    dtype: DType,
}

impl Deserialize for RebootSeed {
    type TypeReader = RebootSeedReader;

    fn reader(dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(RebootSeedReader {
            dtype: dtype.clone(),
        })
    }
}

impl TypeRead for RebootSeedReader {
    type Value = RebootSeed;

    fn read_one<R: std::io::Read>(&self, bytes: R) -> std::io::Result<Self::Value>
    where
        Self: Sized,
    {
        let bytes = bytes.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
        let mut bytes_iter = bytes.iter();

        let dtype = self.dtype.clone();

        let mut dtypes = match dtype {
            DType::Record(r) => {
                let mut dtypes = vec![];
                for field in r {
                    match field.name.as_str() {
                        "last_state" | "experiment_parameters" => {
                            dtypes.push(field.dtype);
                        }

                        _ => (),
                    }
                }
                dtypes
            }
            _ => panic!(),
        };

        let last_state_dtype = dtypes.remove(0);
        let experiment_parameters_dtype = dtypes.remove(0);

        let last_state_dtypes = match last_state_dtype {
            DType::Record(r) => r,
            _ => panic!(),
        };

        assert!(
            vec!["edges", "rng_gen", "performed_updates", "accepted_updates"]
                .iter()
                .all(|name| last_state_dtypes.iter().any(|field| field.name == *name))
        );

        // Deserialize edges
        let edges_bytes = last_state_dtypes[0].dtype.num_bytes().unwrap();
        let edges = bytes_iter
            .by_ref()
            .take(edges_bytes)
            .map(|byte| *byte)
            .collect::<Vec<u8>>();

        // Deserialize rng_gen

        let rng_gen_bytes_number = last_state_dtypes[1].dtype.num_bytes().unwrap();
        let rng_gen_bytes = bytes_iter
            .by_ref()
            .take(rng_gen_bytes_number)
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

        let last_state = LastState {
            edges,
            rng_gen,
            performed_updates,
            accepted_updates,
        };

        let exp_par_reader =
            <ExperimentParameters as Deserialize>::reader(&experiment_parameters_dtype).unwrap();
        let rest_of_bytes = bytes_iter.map(|byte| *byte).collect::<Vec<u8>>();
        let experiment_parameters = exp_par_reader.read_one(rest_of_bytes.as_slice()).unwrap();

        Ok(RebootSeed {
            last_state,
            experiment_parameters,
        })
    }
}

pub struct BackupDataReader {
    dtype: DType,
}

impl Deserialize for BackupData {
    type TypeReader = BackupDataReader;

    fn reader(dtype: &DType) -> Result<Self::TypeReader, npyz::DTypeError> {
        Ok(BackupDataReader {
            dtype: dtype.clone(),
        })
    }
}

impl TypeRead for BackupDataReader {
    type Value = BackupData;

    fn read_one<R: std::io::Read>(&self, bytes: R) -> std::io::Result<Self::Value>
    where
        Self: Sized,
    {
        let mut shape = vec![];
        shape_recursion(&self.dtype, &mut shape);

        let bytes = bytes.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
        let mut bytes_iter = bytes.iter();

        // Deserialize recorded_data
        let recorded_data_length = shape_to_length(&shape);
        let recorded_data = bytes_iter
            .by_ref()
            .take(recorded_data_length)
            .map(|byte| *byte)
            .collect::<Vec<u8>>();

        let recorded_data = Array::from_shape_vec(shape, recorded_data).unwrap();

        Ok(BackupData {
            recorded_data: recorded_data.into(),
        })
    }
}
