#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::{
    fs::{self, File},
    path::Path,
    thread,
};

use ndarray::Axis;
use npyz::{npz, WriterBuilder};
use rs_to_npy::{array_wrapper::ArrayWrapper, dtypeable::DTypeable};
use rs_to_npy_macros::DTypeable;
use z_n_gauge::{
    experiment::experiment::{calculate_correlators, Experiment},
    measurements::{backup::backup_trait::BackUp, measurement_types::save_edges::SaveEdges},
};
use zip::write::FileOptions;

pub fn calc_correlators<P: AsRef<Path> + std::fmt::Display>(run_path: P) {
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

        // let name = format!("{}/{}", run_path, "archive.npz");
        // let file = File::create(name).unwrap();

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

        // let mut zip = zip::ZipWriter::new(file);

        let mut handles = vec![];

        let mut run_counter = 0;
        for backup_files in files_to_merge.into_iter() {
            run_counter += 1;
            handles.push(thread::spawn(move || {
                // let mut backups = vec![];
                let mut correlators = vec![];

                for backup_file in backup_files.iter().take(5) {
                    println!("{:?}", backup_file);
                    let backup = SaveEdges::from_file(backup_file);
                    // backups.push(backup);

                    for (i, edge_config) in backup.edges.data.axis_iter(Axis(0)).enumerate() {
                        let mut reboot_seed = backup.reboot_seed.clone();
                        reboot_seed.last_state.edges = ArrayWrapper {
                            data: edge_config.to_owned().clone(),
                        };

                        let experiment = Experiment::<3>::reboot_experiment(reboot_seed);

                        let correlator = calculate_correlators(&experiment);
                        correlators.push(correlator);

                        println!("run_counter: {:?}, progress:{:?}", run_counter, i);
                    }
                }

                // let merged_backup = SaveEdges::merge(backups);

                // for (i, edge_config) in merged_backup.edges.data.axis_iter(Axis(0)).enumerate() {
                //     // println!("{:?}", edge_config.shape());

                //     if i % 1000 == 0 {
                //         println!("run_id: {:?}, progress: {:?}", run_counter, i);
                //     }

                //     let mut reboot_seed = merged_backup.reboot_seed.clone();
                //     reboot_seed.last_state.edges = ArrayWrapper {
                //         data: edge_config.to_owned().clone(),
                //     };

                //     let experiment = Experiment::<3>::reboot_experiment(reboot_seed);

                //     correlators.push(calculate_correlators(&experiment));
                // }

                let nrows = correlators.len();
                let ncols = correlators[0].len();

                let mut data = vec![];
                for row in correlators.iter() {
                    data.extend_from_slice(row)
                }

                let data = ndarray::Array2::from_shape_vec((nrows, ncols), data).unwrap();
                let data = data.t().to_owned();

                println!("{:?}", data.shape());

                return data;
            }));
        }

        let mut correlators = vec![];
        for handle in handles {
            correlators.push(handle.join().unwrap());
        }

        println!("{:?}", "Done with all threads");
        for corr in correlators.iter() {
            println!("{:?}", corr.shape());
        }

        #[derive(Debug, npyz::Serialize, npyz::Deserialize, DTypeable)]
        struct Correlators {
            corr: ArrayWrapper<[f32; 2]>,
        }

        let zip_name = format!("{}/{}", run_path, "correlators.npz");
        let file = File::create(&zip_name).unwrap();
        let mut zip = zip::ZipWriter::new(file);

        for correlator in correlators.into_iter() {
            let l = correlator.shape()[0];

            let binding = correlator.into_dyn();
            let corr = Correlators {
                corr: ArrayWrapper { data: binding },
            };

            let options = FileOptions::default().large_file(true);

            zip.start_file(npz::file_name_from_array_name(&format!("{}", l)), options)
                .unwrap();

            println!("{:?}", "writing");

            let mut writer = npyz::WriteOptions::new()
                .dtype(corr.generate_dtype())
                .writer(&mut zip)
                .shape(&[1])
                .begin_nd()
                .unwrap();

            writer.extend(vec![corr]).unwrap();
            writer.finish().unwrap();
        }
    }
}

fn main() {
    // attempt at calculating correlators

    // calc_correlators(r"_recordings/Correlators_2024-05-06--13-37-14");

    // read dtype from npy file

    let run_path = "_recordings/Correlators_big_2024-05-06--13-37-14";

    calc_correlators(run_path);

    // see how many bits are in file
}
