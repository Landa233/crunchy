use z_n_gauge::measurements::{
    measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr, zipper::archive,
};

fn main() {
    archive::<SavePloopPloopCorrCorr, _>("_recordings/2024-06-09--11-45-43-Z_7-with-<P>^2");
}
