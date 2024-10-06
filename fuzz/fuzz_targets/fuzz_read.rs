#![no_main]

use libfuzzer_sys::fuzz_target;
use no_std_io2::io::Read;

fuzz_target!(|data: &[u8]| {
    let mut src = data.clone();

    let mut dest: Vec<u8> = Vec::with_capacity(data.len());
    dest.resize(data.len(), 0);

    let _ = src.read_exact(&mut dest[..data.len()]);
});
