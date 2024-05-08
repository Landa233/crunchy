use std::{fs::File, io};

use z_n_gauge::measurements::{
    backup::backup_trait::BackUp, measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr,
};

fn main() {
    let file_path = "_recordings/Z_7_all_measurements/24-L_10_10_10_10-b_3p00-l_0p00/2.npy";

    let file = io::BufReader::new(File::open(file_path).unwrap());
    let npy = npyz::NpyFile::new(file).unwrap();

    println!("{:?}", npy.dtype());

    let backup = <SavePloopPloopCorrCorr as BackUp>::from_file(file_path);
}
