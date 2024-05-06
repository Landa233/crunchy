use z_n_gauge::measurements::{measurement_types::save_edges::SaveEdges, zipper::archive};

fn main() {
    archive::<SaveEdges, _>("_recordings/Correlators_2024-05-02--16-56-11");
}
