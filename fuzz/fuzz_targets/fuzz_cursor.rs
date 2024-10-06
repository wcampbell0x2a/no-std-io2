#![no_main]

extern crate alloc;

use alloc::vec::Vec;

use libfuzzer_sys::fuzz_target;
use no_std_io2::io::{Cursor, Seek, SeekFrom, Write};

fuzz_target!(|data: &[u8]| {
    let len = data.len();
    let mut pos = len / 2;

    let mut dest: Vec<u8> = Vec::with_capacity(len);
    dest.resize(len, 0);

    let mut cursor = Cursor::new(dest);

    cursor.seek(SeekFrom::End(-(pos as i64))).unwrap();

    while pos < len {
        let bytes_written = cursor.get_mut().write(&data[pos..]).unwrap();

        pos += bytes_written;
    }
});
