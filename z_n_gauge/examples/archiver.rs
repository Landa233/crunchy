use z_n_gauge::measurements::{
    measurement_types::ploop_ploopcorr_corr::SavePloopPloopCorrCorr, zipper::archive,
};

fn main() {
    archive::<SavePloopPloopCorrCorr, _>(
        "_recordings/2024-06-19--11-58-06_partial-plus-correlator_L6toL8-2",
    );
}
