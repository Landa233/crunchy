use std::env;
use std::fs;

use z_n_gauge::measurements::settings::ShedulerSettings;

fn main() {
    let file_name = "configs/sweep_settings.json";

    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let sheduler_settings: ShedulerSettings = serde_json::from_str(&contents).unwrap();

    sheduler_settings.write_batch_file(file_name);

    env::var("CRUNCHY_ROOT_DIRECTORY").expect("env variable CRUNCHY_ROOT_DIRECTORY not set");

    let command = "sbatch";
    let args = ["crunchy.sh"];

    use std::process::Command;
    // Execute the command
    let output = Command::new(command)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    // Print the output
    println!("Command executed with output: {:?}", output);
}
