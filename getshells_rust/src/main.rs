#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]
#![feature(read_buf)]
#![feature(hash_raw_entry)]
use std::{
    fmt::Display,
    fs::File,
    io::{self, stdout, BufRead, BufReader, Write},
};

#[cfg(target_feature = "avx")]
use ahash::AHashMap;
#[cfg(not(target_feature = "avx"))]
use std::collections::HashMap;

fn main() {
    const FILE: &str = "passwd";
    // Read buffer size is also a fragile optimization, mess around with capcacities 64 Kib -
    // 1024 Kib to find the best result
    const CHUNK: usize = 64 * 1024;

    let file = File::open(FILE).expect("failed to read {FILE}");
    let file = BufReader::with_capacity(CHUNK, file);
    //
    // Use aHash on AVX enabled platforms, should be faster
    #[cfg(target_feature = "avx")]
    let mut hs = AHashMap::with_capacity(512); // Initial capacity 32 performed the best at the time
                                               // of testing, probably a fragile optimization
    #[cfg(not(target_feature = "avx"))]
    let mut hs = HashMap::with_capacity(512);
    let mut stdout = stdout().lock();
    let _ = file.split(b'\n').try_for_each(|line| -> io::Result<()> {
        let line = line?;
        let colon_idx = line
            .iter()
            .enumerate()
            .rev()
            // This mess finds the first `:` from the end of the line
            .find(|(_, &b)| b == b':')
            .map(|(idx, _)| idx)
            .unwrap_or_default();

        // Only needed if we assume the file can be CRLF encoded
        // (it can't because that'd be weird)
        //
        // if line.last() == Some(&b'\r') {
        //     line.pop();
        // }
        let shell = &line[colon_idx + 1..];
        hs.raw_entry_mut()
            .from_key(shell)
            // .from_key(&line[colon_idx + 1..])
            .and_modify(|_, v| *v += 1)
            .or_insert_with(|| (shell.to_vec(), 1));
        Ok(())
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
