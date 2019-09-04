use super::serial;
use core::slice;
use device_tree::{DeviceTree, Node};

fn init_serial(dt: &Node) {
    if let Ok(compatible) = dt.prop_str("compatible") {
        if compatible == "ns16550a" {
            serial::init(dt);
            return;
        }
    }
    for child in dt.children.iter() {
        init_serial(child);
    }
}

struct DtbHeader {
    _magic: u32,
    size: u32,
}

pub fn init(dtb: usize) {
    let header = unsafe { &*(dtb as *const DtbHeader) };
    let data =
        unsafe { slice::from_raw_parts(dtb as *const u8, u32::from_be(header.size) as usize) };
    if let Ok(dt) = DeviceTree::load(data) {
        init_serial(&dt.root);
        // printing works now
        //println!("Device Tree: {:?}", dt);
    }
}