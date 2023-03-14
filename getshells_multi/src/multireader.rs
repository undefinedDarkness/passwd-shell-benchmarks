#![feature(hash_raw_entry)]
use std::{
    fmt::Display,
    fs::File,
    io::{self, stdout, BufReader, ErrorKind, Read, Seek, Write},
    path::Path,
    thread::scope,
};

use ahash::AHashMap;
use bstr::io::BufReadExt;
use memchr::memrchr;

const PATH: &str = "passwd";

const LINE_FEED: u8 = b'\n';

fn main() {
    let mut args = std::env::args().skip(1);

    let thread_count = match args.next().map(|x| x.parse::<u64>()) {
        Some(Ok(n)) => {
            if n == 0 {
                eprintln!("Thread count(arg1) cannot be zero");
                std::process::exit(2)
            }
            n
        }
        Some(Err(err)) => panic!("Failed to parse the first argument(thread count),{err}"),
        None => num_cpus::get() as u64,
    };

    let file = File::open(PATH).unwrap();

    let read_chunk_size = match args.next().map(|x| x.parse::<u64>()) {
        Some(Ok(n)) => {
            if n == 0 {
                eprintln!("Chunk size(arg2, KiB) cannot be zero");
                std::process::exit(2)
            }
            n
        }
        Some(Err(err)) => panic!("Failed to parse the second argument(chunk size),{err}"),
        None => 64,
    } * 1024;

    let thread_configs = ThreadConfig::generate_chunked(&file, thread_count, LINE_FEED).unwrap();

    let hashmap = scope(|s| {
        let threads: Vec<_> = thread_configs
            .into_iter()
            .map(|config| s.spawn(move || config.run(PATH, read_chunk_size)))
            .collect();
        let mut threads = threads.into_iter();
        let mut hashmap = threads.next().unwrap().join().unwrap().unwrap();
        threads
            .try_for_each(|handle| -> io::Result<()> {
                handle
                    .join()
                    .expect("Failed to join thread")
                    .unwrap()
                    .into_iter()
                    .for_each(|(k, v)| {
                        hashmap
                            .entry(k)
                            .and_modify(|count| *count += v)
                            .or_insert(v);
                    });
                Ok(())
            })
            .unwrap();
        hashmap
    });

    let mut stdout = stdout().lock();
    hashmap.into_iter().for_each(|(shell, count)| {
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

#[derive(Debug)]
struct ThreadConfig {
    start: usize,
    length: usize,
}

impl ThreadConfig {
    pub fn run<P>(self, path: P, chunk_size: u64) -> io::Result<AHashMap<Vec<u8>, u32>>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        file.seek(std::io::SeekFrom::Start(self.start as u64))?;
        let file = file.take(self.length as u64);
        let mut reader = BufReader::with_capacity(chunk_size as usize, file);

        let mut hashmap = AHashMap::with_capacity(32);

        reader.for_byte_line(|line| -> io::Result<bool> {
            if let Some(colon_idx) = memrchr(b':', line) {
                let shell = &line[colon_idx + 1..];
                hashmap
                    .raw_entry_mut()
                    .from_key(shell)
                    .and_modify(|_, v| *v += 1)
                    .or_insert_with(|| (shell.to_vec(), 1));
            };
            Ok(true)
        })?;
        Ok(hashmap)
    }

    pub fn generate_chunked<R: Read + Seek>(
        mut file: R,
        thread_count: u64,
        sep: u8,
    ) -> std::io::Result<Vec<Self>> {
        let size = file.seek(std::io::SeekFrom::End(0))?;

        const LOOKAHEAD_BUMP_SIZE: u64 = 2048;

        let chunk_size = (size / thread_count) as usize;
        let mut buf = Vec::with_capacity(LOOKAHEAD_BUMP_SIZE as usize);

        let mut thread_configs = Vec::with_capacity(thread_count as usize);
        let mut last_end = 0usize;
        for _ in 0..thread_count {
            let start = last_end;
            if start as u64 >= size {
                break;
            }
            let chunk_len = chunk_size.min(size as usize - start);
            let offset = 'inner: {
                let mut offset = 0;
                let Ok(_) = file.seek(std::io::SeekFrom::Start((start + chunk_len) as u64)) else {
                break 'inner offset;
            };
                loop {
                    buf.clear();
                    match file
                        .by_ref()
                        .take(LOOKAHEAD_BUMP_SIZE)
                        .read_to_end(&mut buf)
                    {
                        Ok(0) => {
                            // EOF condition
                            break offset;
                        }
                        Ok(read) => {
                            if let Some(end) = buf.iter().position(|&b| b == sep) {
                                break offset + end;
                            } else {
                                offset += read;
                            }
                        }
                        Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                        Err(err) => Err(err)?,
                    };
                }
            };
            let length = offset + chunk_len;
            thread_configs.push(ThreadConfig { start, length });
            last_end = start + length + 1;
        }
        Ok(thread_configs)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::ThreadConfig;

    #[test]
    fn thread_configs() {
        const COUNT: u64 = 100;
        let input: Vec<_> = (0..COUNT).map(|x| (x % 2) as u8).collect();
        let size = input.len();
        for threads in 1..COUNT {
            let input = Cursor::new(&input);
            let thread_configs = ThreadConfig::generate_chunked(input, threads, b'0').unwrap();
            assert_eq!(
                thread_configs.iter().map(|x| x.length).sum::<usize>(),
                size,
                "All lengths have to add up to the total size"
            );
            assert_eq!(
                {
                    let tc = thread_configs.last().unwrap();
                    tc.start + tc.length
                },
                size,
                "The start position of the last thread config and its length have to add up to the total size"
            )
        }
    }
}
