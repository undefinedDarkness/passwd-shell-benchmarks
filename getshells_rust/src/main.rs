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
    const CHUNK: usize = 128 * 1024;

    let file = BufReader::with_capacity(CHUNK, File::open(FILE).expect("failed to read {FILE}"));
    // Use aHash on AVX enabled platforms, should be faster
    #[cfg(target_feature = "avx")]
    let mut hs = AHashMap::with_capacity(32); // Initial capacity 32 performed the best at the time
                                              // of testing, probably a fragile optimization
    #[cfg(not(target_feature = "avx"))]
    let mut hs = HashMap::with_capacity(32);
    file.split(b'\n')
        .try_for_each(|line| -> io::Result<()> {
            let mut line = line?;
            let colon_idx = line
                .iter()
                .enumerate()
                .rev()
                // This mess finds the first `:` from the end of the line
                .find(|(_, &b)| b == b':')
                .map(|(idx, _)| idx)
                .unwrap_or_default();
            line.drain(0..=colon_idx); // Remove all elements up to and including `:`

            // Only needed if we assume the file can be CRLF encoded
            // (it can't because that'd be weird)
            //
            // if line.last() == Some(&b'\r') {
            //     line.pop();
            // }
            hs.entry(line).and_modify(|x| *x += 1u32).or_insert(1);
            Ok(())
        })
        .unwrap();

    let mut stdout = stdout().lock();

    hs.into_iter().for_each(|(shell, count)| {
        let _ = stdout.write_fmt(format_args!("{}: {count}", UnsafeBytes(shell)));
    });
}

#[repr(transparent)]
struct UnsafeBytes(Vec<u8>);
impl Display for UnsafeBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { std::str::from_utf8_unchecked(&self.0) })
    }
}
