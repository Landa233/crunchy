use byteorder::{LittleEndian, WriteBytesExt};
use npyz::{AutoSerialize, Serialize, TypeWrite};

use super::backup::{reboot_seed_dtype, BackUp, ExperimentParameters, RebootSeed, RunInfo};

pub struct BackupWriter {}

impl Serialize for BackUp {
    type TypeWriter = BackupWriter;

    fn writer(_dtype: &npyz::DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(BackupWriter {})
    }
}

pub struct RebootSeedWriter {}

impl Serialize for RebootSeed {
    type TypeWriter = RebootSeedWriter;

    fn writer(_dtype: &npyz::DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(RebootSeedWriter {})
    }
}

impl TypeWrite for RebootSeedWriter {
    type Value = RebootSeed;

    fn write_one<W: std::io::Write>(
        &self,
        mut writer: W,
        value: &Self::Value,
    ) -> std::io::Result<()>
    where
        Self: Sized,
    {
        // Write Last State
        for &edge in value.last_state.edges.iter() {
            writer.write_u8(edge).unwrap();
        }

        let rng_bytes = bincode::serialize(&value.last_state.rng_gen).unwrap();
        for &byte in rng_bytes.iter() {
            writer.write_u8(byte).unwrap();
        }

        writer
            .write_u64::<LittleEndian>(value.last_state.performed_updates)
            .unwrap();
        writer
            .write_u64::<LittleEndian>(value.last_state.accepted_updates)
            .unwrap();

        // Write Experiment Parameters
        let experiment_parameters_dtype = <ExperimentParameters as AutoSerialize>::default_dtype();
        let experiment_parameters_writer =
            <ExperimentParameters as Serialize>::writer(&experiment_parameters_dtype).unwrap();

        experiment_parameters_writer.write_one(&mut writer, &value.experiment_parameters)?;

        Ok(())
    }
}

impl TypeWrite for BackupWriter {
    type Value = BackUp;

    fn write_one<W: std::io::Write>(
        &self,
        mut writer: W,
        value: &Self::Value,
    ) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let reboot_seed_dtype = reboot_seed_dtype(&value.reboot_seed);
        let reboot_seed_writer = <RebootSeed as Serialize>::writer(&reboot_seed_dtype).unwrap();
        reboot_seed_writer.write_one(&mut writer, &value.reboot_seed)?;

        // Serialize recorded data
        for &recording in value.backup_data.recorded_data.iter() {
            writer.write_u8(recording).unwrap();
        }

        let run_info_dtype = <RunInfo as AutoSerialize>::default_dtype();
        let run_info_writer = <RunInfo as Serialize>::writer(&run_info_dtype).unwrap();
        run_info_writer.write_one(&mut writer, &value.run_info)?;

        Ok(())
    }
}
