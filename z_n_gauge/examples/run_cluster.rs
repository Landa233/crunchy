use std::env;
use std::fs;

use z_n_gauge::sheduler::phase_diagram::phase_diagram_sweep::PhaseDiagram;

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

    let phase_diagram: PhaseDiagram<3> = serde_json::from_str(&contents).unwrap();

    phase_diagram.write_batch_file(file_name);

    // run batch file
    let output = std::process::Command::new("sh")
        .arg(file_name)
        .status()
        .unwrap();

    if output.success() {
        println!("Successfully ran batch file");
    } else {
        println!("Failed to run batch file");
    }
}
