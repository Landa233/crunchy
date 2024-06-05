use z_n_gauge::measurements::{
    measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr, zipper::archive,
};

fn main() {
    archive::<SavePloopPloopCorrCorr, _>("_recordings/2024-05-29--17-19-41");
}
