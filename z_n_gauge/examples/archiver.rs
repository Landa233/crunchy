use z_n_gauge::measurements::{
    measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr, zipper::archive,
};

fn main() {
    archive::<SavePloopPloopCorrCorr, _>("_recordings/2024-06-27--11-52-35_split-correlators");
}
