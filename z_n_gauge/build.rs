include!("src/measurements/settings.rs");

use std::{fs, path::Path};

fn main() {
    let constants_path = Path::new("src").join("constants.rs");

    // Read Sweep ZORDER
    let contents = fs::read_to_string("../configs/sweep_settings.json").unwrap();
    let sweep_settings: ShedulerSettings =
        serde_json::from_str::<ShedulerSettings>(&contents).unwrap();
    let sweep_zorder = sweep_settings.z_order;

    // Read Relaunch ZORDER
    let contents = fs::read_to_string("../configs/relaunch_settings.json").unwrap();
    let relaunch_settings: RelaunchSettings = serde_json::from_str(&contents).unwrap();
    let relaunch_zorder = relaunch_settings.z_order;

    let mut file = File::create(constants_path).unwrap();

    let generated_code = format!(
        r#"
        pub const SWEEP_ZORDER: usize = {}; 
        pub const RELAUNCH_ZORDER: usize = {};"#,
        sweep_zorder, relaunch_zorder
    );

    file.write_all(generated_code.as_bytes()).unwrap();
}
