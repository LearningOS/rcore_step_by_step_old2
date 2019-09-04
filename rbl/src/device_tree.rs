use core::slice;
use alloc::format;
use device_tree::{DeviceTree};

struct DtbHeader {
    _magic: u32,
    size: u32,
}

pub fn init(dtb: usize) {
    let header = unsafe {
        &*(dtb as *const DtbHeader)
    };
    let data = unsafe {
        slice::from_raw_parts(dtb as *const u8, header.size as usize)
    };
    let dt = DeviceTree::load(data);
    let orz = format!("{:?}", dt);
    loop {

    }
}