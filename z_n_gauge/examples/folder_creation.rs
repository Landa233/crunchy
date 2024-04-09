use std::fs;

fn main() {
    let p = "_test/folder/one";

    let a = fs::create_dir_all(p).unwrap();

    let file_name = "MyFile.txt";

    fs::File::create(format!("{}/{}", p, file_name)).unwrap();
}
