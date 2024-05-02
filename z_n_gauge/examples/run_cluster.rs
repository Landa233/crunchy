use std::env;
use std::fs;

use z_n_gauge::measurements::sheduler::ShedulerSettings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file name as a command line argument");
        return;
    }

    let file_name = &args[1];

    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    let sheduler_settings: ShedulerSettings<3> = serde_json::from_str(&contents).unwrap();

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
