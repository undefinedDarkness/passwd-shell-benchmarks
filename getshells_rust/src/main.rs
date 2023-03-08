use std::{
    fs::File,
    io::{self, stdout, BufRead, BufReader, Write},
};

#[cfg(target_feature = "avx")]
use ahash::AHashMap;
#[cfg(not(target_feature = "avx"))]
use std::collections::HashMap;

fn main() {
    const FILE: &str = "passwd";
    const CHUNK: usize = 256 * 1024;

    let file = BufReader::with_capacity(CHUNK, File::open(FILE).expect("failed to read {FILE}"));
    #[cfg(target_feature = "avx")]
    let mut hs = AHashMap::with_capacity(512);
    #[cfg(not(target_feature = "avx"))]
    let mut hs = HashMap::with_capacity(512);
    file.split(b'\n')
        .try_for_each(|line| -> io::Result<()> {
            let mut line = line?;
            let colon_idx = line
                .iter()
                .enumerate()
                .rev()
                .find(|(_, &b)| b == b':')
                .map(|(idx, _)| idx)
                .unwrap_or_default();
            line.drain(0..=colon_idx);
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            line.shrink_to_fit();
            hs.entry(line).and_modify(|x| *x += 1u32).or_insert(1);
            Ok(())
        })
        .unwrap();
    let mut stdout = stdout().lock();

    hs.into_iter().for_each(|(shell, count)| {
        let _ = stdout.write_all(&shell);
        let _ = writeln!(&mut stdout, ": {count}");
    })
}
