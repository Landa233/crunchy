use std::{
    fs::{self, File},
    path::Path,
};

use npyz::{npz, WriterBuilder};
use zip::write::FileOptions;

use crate::experiment::backup::backup::{backup_dtype, BackUp};

pub fn archive<P: AsRef<Path> + std::fmt::Display>(run_path: P) {
    let mut counter = 0;

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

        let mut min_file_name = vec![];
        let mut files_to_merge = vec![];

        for file_name in file_names.iter() {
            if let Ok(backup_file_names) = fs::read_dir(format!("{}/{}", run_path, file_name)) {
                let mut backup_files: Vec<String> = vec![];
                for backup_file in backup_file_names {
                    backup_files.push(backup_file.unwrap().file_name().into_string().unwrap());
                }

                let mut backups = vec![];
                for backup_file in backup_files {
                    let number: u32 = backup_file.split('.').collect::<Vec<&str>>()[0]
                        .parse()
                        .unwrap();
                    backups.push((number, backup_file));
                }

                backups.sort_by(|a, b| a.0.cmp(&b.0));

                min_file_name.push(backups.last().unwrap().0);
                let mut temp = vec![];
                for b in backups.iter() {
                    temp.push(format!("{}/{}/{}", run_path, file_name, b.1));
                }
                files_to_merge.push(temp);
            }
        }

        let min = min_file_name.iter().min();
        for files in files_to_merge.iter_mut() {
            files.truncate(*min.unwrap() as usize);
        }

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default().large_file(true);

        for backup_files in files_to_merge.iter() {
            let mut backups = vec![];
            for backup_file in backup_files {
                let backup = BackUp::from_file(backup_file);
                backups.push(backup);
            }

            let merged_backup = BackUp::merge_backups(backups);

            let file_name = backup_files[0].split('/').collect::<Vec<&str>>()[2];
            println!("{:?}", file_name);
            counter += 1;
            println!("{:?}", counter);

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
        zip.finish().unwrap();
    }
}
