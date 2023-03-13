#![feature(hash_raw_entry)]
use std::{
    fmt::Display,
    fs::File,
    io::{stdout, Read, Write},
    sync::{atomic::AtomicBool, mpsc},
    thread::scope,
};

use ahash::AHashMap;

static STOPPED: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut file = File::open("passwd").expect("Failed to open passwd");
    // let file = BufReader::with_capacity(1024 * 64, file);
    const CHUNK_DEFAULT: usize = 64;
    const THREAD_COUNT_DEFAULT: u64 = 9;

    let mut args = std::env::args().skip(1);
    let chunk = match args.next().map(|x| x.parse::<usize>()) {
        Some(Ok(n)) => n,
        Some(Err(err)) => panic!("Failed to parse the first argument(read chunk size),{err}"),
        None => CHUNK_DEFAULT,
    } * 1024;

    let thread_count = match args.next().map(|x| x.parse::<u64>()) {
        Some(Ok(n)) => n,
        Some(Err(err)) => panic!("Failed to parse the second argument(thread count),{err}"),
        None => THREAD_COUNT_DEFAULT,
    };

    let mut buf = Vec::with_capacity(chunk);
    let hashmap = scope(|s| {
        let threads: Vec<_> = (0..thread_count)
            .map(|_| {
                let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(16);
                // let (tx, rx) = mpsc::channel::<Vec<u8>>();

                let mut hashmap = AHashMap::with_capacity(32);

                let handle = s.spawn(move || {
                    loop {
                        if let Ok(message) = rx.try_recv() {
                            message.split(|b| b == &b'\n').for_each(|line| {
                                if let Some(colon_idx) = line
                                    .iter()
                                    .enumerate()
                                    .rev()
                                    // This mess finds the first `:` from the end of the line
                                    .find(|(_, &b)| b == b':')
                                    .map(|(idx, _)| idx + 1)
                                {
                                    let shell = &line[colon_idx..];
                                    hashmap
                                        .raw_entry_mut()
                                        .from_key(shell)
                                        .and_modify(|_, v| *v += 1)
                                        .or_insert_with(|| (shell.to_vec(), 1));
                                };
                            });
                        } else {
                            if STOPPED.load(std::sync::atomic::Ordering::Acquire) {
                                break;
                            }
                        };
                    }
                    hashmap
                });
                (handle, tx)
            })
            .collect();
        let mut thread_distributor = threads.iter().cycle();
        // let initialized = start.elapsed().as_nanos();
        loop {
            match Read::by_ref(&mut file)
                .take(chunk as u64)
                .read_to_end(&mut buf)
            {
                Ok(0) => {
                    // EOF case
                    thread_distributor.next().unwrap().1.send(buf).expect("Failed to send, the channel has been deallocated while the loop hasn't stopped somehow");
                    break;
                }
                Ok(_) => {
                    // Successful read
                    // Find last newline
                    let Some(found) =
                    buf
                        .iter()
                        .rev()
                        .position(|b| b == &b'\n') else {
                            continue;
                        };
                    let len = buf.len() - found;

                    // Get the rest
                    let rest = buf.get(len..).unwrap_or_default();
                    // Create a new buffer for the rest of the read bytes
                    let mut new_buf = Vec::with_capacity(chunk);
                    new_buf.extend_from_slice(rest);
                    // Remove the rest from the main buffer
                    buf.truncate(len.max(1) - 1);
                    std::mem::swap(&mut new_buf, &mut buf);
                    // At this point, new_buf is a chunk of input ready to be processed
                    // println!("{:?}", unsafe { std::str::from_utf8_unchecked(&new_buf) })
                    thread_distributor.next().unwrap().1.send(new_buf).expect("Failed to send, the channel has been deallocated while the loop hasn't stopped somehow");
                }
                Err(err) => panic!("{}", err),
            }
        }
        // let read = start.elapsed().as_nanos();
        STOPPED.store(true, std::sync::atomic::Ordering::Release);
        let mut threads = threads.into_iter();
        let mut hashmap = threads
            .next()
            .expect("Somehow failed to get a single thread????")
            .0
            .join()
            .expect("Failed to join the first thread after all threads have been stopped?");
        threads.for_each(|(handle, _)| {
            handle
                .join()
                .expect("Failed to join a thread, this shouldn't happen")
                .into_iter()
                .for_each(|(k, v)| {
                    hashmap
                        .entry(k)
                        .and_modify(|count| *count += v)
                        .or_insert(v);
                })
        });
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
