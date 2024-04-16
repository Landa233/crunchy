use std::{
    fs::{self, File},
    path::Path,
};

use npyz::{npz, WriterBuilder};
use zip::write::FileOptions;

use crate::experiment::backup::backup::{backup_dtype, BackUp};

pub fn archive<P: AsRef<Path> + std::fmt::Display>(run_path: P) {
    let run_path = run_path.to_string();
    if let Ok(entries) = fs::read_dir(&run_path) {
        let mut file_names = vec![];
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    file_names.push(file_name);
                }
            }
        }

        let name = format!("{}/{}", run_path, "archive.npz");
        let file = File::create(name).unwrap();

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default().large_file(true);

        for file_name in file_names {
            if let Ok(backup_file_names) = fs::read_dir(format!("{}/{}", run_path, file_name)) {
                let mut backup_files: Vec<String> = vec![];
                for backup_file in backup_file_names {
                    backup_files.push(backup_file.unwrap().file_name().into_string().unwrap());
                }

                backup_files.sort();

                let mut backups = vec![];
                for backup_file in backup_files {
                    let backup =
                        BackUp::from_file(format!("{}/{}/{}", run_path, file_name, backup_file));
                    backups.push(backup);
                }

                let merged_backup = BackUp::merge_backups(backups);

                zip.start_file(
                    npz::file_name_from_array_name(&format!("{}", file_name)),
                    options,
                )
                .unwrap();

                let mut writer = npyz::WriteOptions::new()
                    .dtype(backup_dtype(&merged_backup))
                    .writer(&mut zip)
                    .shape(&[1])
                    .begin_nd()
                    .unwrap();

                writer.extend(vec![merged_backup]).unwrap();

                writer.finish().unwrap();
            }
        }
        zip.finish().unwrap();
    }
}
