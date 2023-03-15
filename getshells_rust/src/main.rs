#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]
#![feature(read_buf)]
#![feature(hash_raw_entry)]
use std::{
    fmt::Display,
    fs::File,
    io::{self, stdout, Write},
};

use ahash::AHashMap;
use bstr::ByteSlice;
use memchr::memrchr;
use memmap2::Mmap;

fn main() {
    const FILE: &str = "passwd";
    // Read buffer size is also a fragile optimization, mess around with capcacities 64 Kib -
    // 1024 Kib to find the best result
    const CHUNK: usize = 64 * 1024;

    let file = File::open(FILE).expect("failed to read {FILE}");
    let mapped = unsafe { Mmap::map(&file).unwrap() };
    //
    // Use aHash on AVX enabled platforms, should be faster
    let mut hs = AHashMap::with_capacity(32); // Initial capacity 32 performed the best at the time
                                              // of testing, probably a fragile optimization
    let mut stdout = stdout().lock();
    mapped.lines().for_each(|line| {
        let colon_idx = memrchr(b':', line).unwrap_or(0);

        let shell = &line[colon_idx + 1..];
        hs.raw_entry_mut()
            .from_key(shell)
            // .from_key(&line[colon_idx + 1..])
            .and_modify(|_, v| *v += 1)
            .or_insert_with(|| (shell.to_vec(), 1));
    });

    hs.into_iter().for_each(|(shell, count)| {
        let _ = stdout.write_fmt(format_args!("{}: {count}\n", UnsafeBytes(&shell)));
    });
}

#[repr(transparent)]
struct UnsafeBytes<'a>(&'a [u8]);
impl<'a> Display for UnsafeBytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { std::str::from_utf8_unchecked(&self.0) })
    }
}
