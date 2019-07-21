extern crate device_tree;

use std::fs;
use std::io::Read;
use std::io::Write;

fn main() {
    // read file into memory
    let mut input = fs::File::open("sample.dtb").unwrap();
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).unwrap();

    let dt = device_tree::DeviceTree::load(buf.as_slice()).unwrap();
    println!("{:?}", dt);

    let dtb = dt.store().unwrap();
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.dtb")
        .unwrap();
    output.write_all(&dtb).unwrap();
}
