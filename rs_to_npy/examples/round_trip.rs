use std::{fs, io};

use ndarray::Array;
use npyz::{Deserialize, Serialize, WriterBuilder};
use rs_to_npy::{array_wrapper::ArrayWrapper, dtypeable::DTypeable};
use rs_to_npy_macros::DTypeable;

#[derive(DTypeable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Foo {
    a: i32,
    b: i32,
    v: ArrayWrapper<u32>,
}

#[derive(DTypeable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Bar {
    w: ArrayWrapper<Foo>,
}

fn main() {
    let a = Array::from_vec(vec![1, 2, 3]);

    let foo1 = Foo {
        a: 1,
        b: 2,
        v: ArrayWrapper { data: a.into_dyn() },
    };

    let foo2 = Foo {
        a: 3,
        b: 4,
        v: ArrayWrapper {
            data: Array::from_vec(vec![10, 11, 12]).into_dyn(),
        },
    };

    let bar = Bar {
        w: ArrayWrapper {
            data: Array::from_vec(vec![foo1, foo2]).into_dyn(),
        },
    };

    let bar_preserialized = bar.clone();

    let mut file = io::BufWriter::new(fs::File::create("round_trip.npy").unwrap());
    let mut writer = npyz::WriteOptions::new()
        .dtype(bar.generate_dtype())
        .writer(&mut file)
        .begin_1d()
        .unwrap();

    writer.push(&bar).unwrap();
    writer.finish().unwrap();

    let bytes = std::fs::read("round_trip.npy").unwrap();
    let reader = npyz::NpyFile::new(&bytes[..]).unwrap();

    let deserialized_backup: Vec<Bar> = reader.into_vec().unwrap();
    let deserialized = deserialized_backup.first().unwrap();

    assert_eq!(*deserialized, bar_preserialized);
}
