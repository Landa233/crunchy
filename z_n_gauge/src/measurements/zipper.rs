use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use npyz::{npz, WriterBuilder};
use zip::write::FileOptions;

use super::backup::backup_trait::BackUp;

pub fn archive<BackUpType: BackUp, P: AsRef<Path> + std::fmt::Display>(run_path: P) {
    let mut counter = 0;

    println!("{}", run_path);

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

        // let min = min_file_name.iter().min();
        // for files in files_to_merge.iter_mut() {
        //     files.truncate(*min.unwrap() as usize);
        // }

        // Store length of files in hashmap indexed by lattice size
        let mut lattice_sizes = HashMap::new();

        fn get_lattice_size(file_name: &str) -> Vec<usize> {
            let lattice_size = file_name
                .split_once("L_")
                .unwrap()
                .1
                .split_once("-")
                .unwrap()
                .0;
            lattice_size
                .split('_')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        }

        for files in files_to_merge.iter() {
            let first_file = files[0].clone();
            let lattice_size = get_lattice_size(&first_file);

            // check if key is already in hashmap
            if lattice_sizes.contains_key(&lattice_size) {
                // if key is in hashmap, increment value by 1
                let value: &mut Vec<usize> = lattice_sizes.get_mut(&lattice_size).unwrap();
                value.push(files.len());
            } else {
                // if key is not in hashmap, add key with value 1
                lattice_sizes.insert(lattice_size, vec![files.len()]);
            }
        }

        let mut truncation_map = HashMap::new();

        for (key, value) in lattice_sizes.iter() {
            let min = value.iter().min().unwrap();
            truncation_map.insert(key, *min);
        }

        // truncate files

        for files in files_to_merge.iter_mut() {
            let first_file = files[0].clone();
            let lattice_size = get_lattice_size(&first_file);
            let truncation = truncation_map.get(&lattice_size).unwrap();
            files.truncate(*truncation);
        }

        for file in files_to_merge.iter() {
            println!("{:?}", file.len());
        }

        let mut zip = zip::ZipWriter::new(file);

        for backup_files in files_to_merge.iter() {
            let mut backups = vec![];
            for backup_file in backup_files {
                println!("{:?}", backup_file);
                let backup = BackUpType::from_file(backup_file);
                backups.push(backup);
            }

            let merged_backup = BackUpType::merge(backups);

            let file_name = backup_files[0].split('/').collect::<Vec<&str>>()[2];
            println!("{:?}", file_name);
            counter += 1;
            println!("{:?}", counter);

            let options = FileOptions::default().large_file(true);
            zip.start_file(
                npz::file_name_from_array_name(&format!("{}", file_name)),
                options,
            )
            .unwrap();

            println!("{:?}", "writing");

            let mut writer = npyz::WriteOptions::new()
                .dtype(merged_backup.generate_dtype())
                .writer(&mut zip)
                .shape(&[1])
                .begin_nd()
                .unwrap();

            writer.extend(vec![merged_backup]).unwrap();
            writer.finish().unwrap();

            println!("{:?}", "written");
        }
        zip.finish().unwrap();
    }
}
