use std::{fmt::Debug, fs, io, path::Path};

use npyz::{Deserialize, Serialize, WriterBuilder};
use rs_to_npy::dtypeable::DTypeable;

pub trait BackUp: Sized + DTypeable + Serialize + Deserialize + Clone + PartialEq + Debug {
    type BackupData;

    fn new(backup_data: Self::BackupData) -> Self;

    fn merge(backups: Vec<Self>) -> Self;

    fn from_file<P>(file_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let bytes = std::fs::read(file_path).unwrap();
        let npy_file = npyz::NpyFile::new(&bytes[..]).unwrap();

        let deserialized_backup: Vec<Self> = npy_file.into_vec().unwrap();
        deserialized_backup.first().unwrap().clone()
    }

    fn to_file<P>(&self, file_path: P)
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
}
