#![feature(hash_raw_entry)]
use std::{
    fmt::Display,
    fs::OpenOptions,
    io::{stdout, Write},
    sync::{atomic::AtomicBool, mpsc},
    thread::scope,
};

use ahash::AHashMap;
use bstr::ByteSlice;
use memchr::{memchr, memrchr};
use memmap2::MmapOptions;

static STOPPED: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut args = std::env::args().skip(1);

    let file = OpenOptions::new().read(true).open("passwd").unwrap();
    let mapped = unsafe { MmapOptions::new().map(&file).unwrap() };

    let thread_count = match args.next().map(|x| x.parse::<usize>()) {
        Some(Ok(n)) => {
            if n == 0 {
                eprintln!("Thread count(arg1) cannot be zero");
                std::process::exit(2)
            }
            n
        }
        Some(Err(err)) => panic!("Failed to parse the first argument(thread count),{err}"),
        None => num_cpus::get_physical(),
    };

    let chunk = match args.next().map(|x| x.parse::<usize>()) {
        Some(Ok(n)) => {
            if n == 0 {
                eprintln!("Thread chunk size(arg2, KiB) cannot be zero");
                std::process::exit(2)
            }
            n
        }
        Some(Err(err)) => {
            panic!("Failed to parse the first argument(thread chunk size, KiB),{err}")
        }
        None => 64,
    } * 1024;

    let _ = mapped.advise(memmap2::Advice::Sequential);
    // let _ = mapped.lock();
    let map: &[u8] = &mapped;

    let hashmap = scope(|s| {
        let threads: Vec<_> = (0..thread_count)
            .map(|_| {
                let (tx, rx) = mpsc::sync_channel::<&[u8]>(16);

                let mut hashmap = AHashMap::with_capacity(32);

                let handle = s.spawn(move || {
                    loop {
                        if let Ok(message) = rx.try_recv() {
                            message.lines().for_each(|line| {
                                let colon_idx = memrchr(b':', line).unwrap_or(0);
                                let shell = &line[colon_idx + 1..];
                                hashmap
                                    .raw_entry_mut()
                                    .from_key(shell)
                                    .and_modify(|_, v| *v += 1)
                                    .or_insert_with(|| (shell.to_vec(), 1));
                            });
                            drop(message)
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
        let mut start = 0;
        while start < map.len() {
            let current = &mapped[start..];
            let len = current.len();
            let end = if len > chunk {
                chunk + memchr(b'\n', &current[chunk..]).unwrap_or(0)
            } else {
                len
            };
            let buf: &[u8] = &current[..end];
            thread_distributor.next().unwrap().1.send(buf).expect("Failed to send, the channel has been deallocated while the loop hasn't stopped somehow");
            start += end + 1;
        }
        // loop {
        //     match Read::by_ref(&mut file)
        //         .take(chunk as u64)
        //         .read_to_end(&mut buf)
        //     {
        //         Ok(0) => {
        //             // EOF case
        //             break;
        //         }
        //         Ok(_) => {
        //             // Successful read
        //             // Find last newline
        //             let Some(found) =
        //             buf
        //                 .iter()
        //                 .rev()
        //                 .position(|b| b == &b'\n') else {
        //                     continue;
        //                 };
        //             let len = buf.len() - found;
        //
        //             // Get the rest
        //             let rest = buf.get(len..).unwrap_or_default();
        //             // Create a new buffer for the rest of the read bytes
        //             let mut new_buf = Vec::with_capacity(chunk);
        //             new_buf.extend_from_slice(rest);
        //             // Remove the rest from the main buffer
        //             buf.truncate(len.max(1) - 1);
        //             std::mem::swap(&mut new_buf, &mut buf);
        //             // At this point, new_buf is a chunk of input ready to be processed
        //             // println!("{:?}", unsafe { std::str::from_utf8_unchecked(&new_buf) })
        //             thread_distributor.next().unwrap().1.send(new_buf).expect("Failed to send, the channel has been deallocated while the loop hasn't stopped somehow");
        //         }
        //         Err(err) if err.kind() == ErrorKind::Interrupted => continue,
        //         Err(err) => panic!("{}", err),
        //     }
        // }
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
