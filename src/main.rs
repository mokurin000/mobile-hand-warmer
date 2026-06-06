#![feature(random)]

use std::random::{DefaultRandomSource, RandomSource};

#[repr(align(128))]
struct Aligned<T>(T);

fn payload_per_thread(payload: fn()) {
    std::iter::once(payload)
        .cycle()
        .take(num_cpus::get())
        .map(std::thread::spawn)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| _ = handle.join());
}

fn main() {
    let payload = || {
        let mut buf = Aligned::<_>([0u8; 128]);
        loop {
            DefaultRandomSource.fill_bytes(&mut buf.0);
        }
    };

    #[cfg(feature = "sha")]
    let payload = if std::env::args().find(|arg| arg == "--sha").is_some() {
        payload
    } else {
        || {
            // 8 MiB per thread
            let mut buf = Aligned::<_>([0u8; 8 * 1024 * 1024]);
            DefaultRandomSource.fill_bytes(&mut buf.0);

            loop {
                std::hint::black_box(ring::digest::digest(&ring::digest::SHA512, &buf.0));
            }
        }
    };

    payload_per_thread(payload);
}
