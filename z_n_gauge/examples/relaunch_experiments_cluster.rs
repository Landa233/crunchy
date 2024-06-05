use std::fs;
use std::fs::File;
use std::io::Write;

use z_n_gauge::constants::RELAUNCH_ZORDER;
use z_n_gauge::measurements::settings::ClusterQueue;
use z_n_gauge::measurements::settings::ClusterSettings;
use z_n_gauge::measurements::settings::RelaunchSettings;
use z_n_gauge::measurements::sheduler::experiments_to_relaunch;

fn write_batch_file<const ZORDER: usize>(cluster_settings: RelaunchSettings, config_file: &str) {
    let number_of_threads = cluster_settings.max_threads;

    let ClusterSettings {
        cluster_time,
        ram,
        temporary_storage,
        email,
        // path_to_executable,
        queue,
    } = cluster_settings.cluster_settings;

    let folder_path = cluster_settings.folder_path;

    let experiments = experiments_to_relaunch::<&str>(folder_path.as_ref());

    let number_of_threads = number_of_threads.min(experiments.0.len());

    let content = format!(
        "#!/bin/bash\n\n# Request resources:\n\
        #SBATCH -c {}\n\
        #SBATCH --time={}-{}:{}:{}\n\
        #SBATCH --mem={}G\n\
        #SBATCH --gres=tmp:{}G\n\
        #SBATCH --mail-user={}\n\
        #SBATCH --mail-type=ALL\n\
        #SBATCH -p {}\n\n\n\
        #Commands to be run:\n\
        cargo run --manifest-path z_n_gauge/Cargo.toml --release --example relaunch_experiments {}",
        number_of_threads,
        cluster_time.days,
        cluster_time.hours,
        cluster_time.minutes,
        cluster_time.seconds,
        ram,
        temporary_storage,
        email,
        match queue {
            ClusterQueue::Test => "test",
            ClusterQueue::Shared => "shared",
        },
        // path_to_executable,
        config_file
    );

    let file_path = "crunchy_relaunch.sh";
    let mut file = File::create(file_path).unwrap();

    file.write_all(content.as_bytes()).unwrap();
}

fn main() {
    let file_name = "configs/relaunch_settings.json";

    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let relaunch_settings: RelaunchSettings = serde_json::from_str(&contents).unwrap();

    write_batch_file::<RELAUNCH_ZORDER>(relaunch_settings, file_name);

    let command = "sbatch";
    let args = ["crunchy_relaunch.sh"];

    use std::process::Command;
    // Execute the command
    let output = Command::new(command)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    // Print the output
    println!("Command executed with output: {:?}", output);
}
