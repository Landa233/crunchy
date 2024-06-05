use std::{fs, io, path::Path};

use npyz::{Deserialize, Serialize, WriterBuilder};
use rs_to_npy::dtypeable::DTypeable;

use crate::measurements::settings::HaltingCondition;

use super::backup_fragments::{RebootSeed, RunInfo};

use core::fmt::Debug;

pub trait BackUp: Sized + DTypeable + Serialize + Deserialize + Clone + PartialEq + Debug {
    type BackupData<'a, const ZORDER: usize>;

    fn new<'a, const ZORDER: usize>(backup_data: Self::BackupData<'a, ZORDER>) -> Self;

    fn merge(backups: Vec<Self>) -> Self;

    fn execute<const ZORDER: usize>(exec_par: ExecutorParameters, reboot_seed: RebootSeed);

    fn from_file<P>(file_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let bytes = std::fs::read(&file_path).unwrap();
        let npy_file = npyz::NpyFile::new(&bytes[..]).unwrap();

        let deserialized_backup: Result<Vec<Self>, io::Error> = npy_file.into_vec();

        let res = match deserialized_backup {
            Ok(deserialized_backup) => deserialized_backup.first().unwrap().clone(),
            Err(_) => panic!(
                "Error deserializing backup {}",
                file_path.as_ref().display()
            ),
        };

        res
    }

    fn write_to_file<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        let mut file = io::BufWriter::new(fs::File::create(file_path).unwrap());
        let mut writer = npyz::WriteOptions::new()
            .dtype(self.generate_dtype())
            .writer(&mut file)
            .begin_1d()
            .unwrap();

        writer.push(self).unwrap();
        writer.finish().unwrap();
    }

    fn reboot_seed(&self) -> RebootSeed;
    fn run_info(&self) -> RunInfo;
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct ExecutorParameters {
    pub halting_condition: HaltingCondition,
    pub recording_skip: u32,
    pub recordings_until_backup: u32,
    pub run_id: u32,
    pub parent_path: String,
    pub backup_number: u32,
}
