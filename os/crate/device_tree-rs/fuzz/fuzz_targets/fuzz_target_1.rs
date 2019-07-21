#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate device_tree;

use device_tree::DeviceTree;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(device_tree) = DeviceTree::load(data) {
        let res = format!("{:?}", device_tree);
        assert!(res.len() > 0);
    }
});
