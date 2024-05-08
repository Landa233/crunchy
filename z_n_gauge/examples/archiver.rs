use z_n_gauge::measurements::{
    measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr, zipper::archive,
};

fn main() {
    archive::<SavePloopPloopCorrCorr, _>("_recordings/Z_7_all_measurements_2024-05-07--20-58-09");
}
