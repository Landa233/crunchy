use npyz::WriterBuilder;
use rand::Rng;
use rs_to_npy::dtypeable::DTypeable;
use z_n_gauge::backups::rng_gen_wrapper::{self, RngGenWrapper};

fn main() {
    let mut rng_gen_wrapper = rng_gen_wrapper::RngGenWrapper {
        rng_gen: rand_pcg::Pcg64Mcg::new(0),
    };

    // Create Copy
    let rng_copy = rng_gen_wrapper.clone();

    // Advance the RNG
    let _: f32 = rng_gen_wrapper.rng_gen.gen();

    let file = std::fs::File::create("_backup_fragments_test.npy").unwrap();
    let mut writer = npyz::WriteOptions::new()
        .dtype(rng_gen_wrapper.generate_dtype())
        .writer(file)
        .begin_1d()
        .unwrap();

    writer.push(&rng_gen_wrapper).unwrap();
    writer.finish().unwrap();

    let bytes = std::fs::read("_backup_fragments_test.npy").unwrap();
    let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

    let deserialized_backup: Vec<RngGenWrapper> = reader.into_vec().unwrap();
    let deserialized = deserialized_backup.first().unwrap();

    // Should fail since the RNG of the copy is not advanced
    assert_eq!(*deserialized, rng_copy);
}
