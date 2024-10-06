#![no_main]

extern crate alloc;

use alloc::vec::Vec;

use libfuzzer_sys::fuzz_target;
use no_std_io2::io::Write;

fuzz_target!(|data: &[u8]| {
    let mut pos = 0;
    let mut buffer = Vec::with_capacity(data.len()); 
    
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
});
