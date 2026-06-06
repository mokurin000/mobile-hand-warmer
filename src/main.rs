#![feature(random)]

use std::random::{DefaultRandomSource, RandomSource};

#[repr(align(128))]
struct Aligned<T>(T);

fn main() {
    std::iter::once(|| {
        let mut buf = Aligned::<_>([0u8; 128]);
        loop {
            DefaultRandomSource.fill_bytes(&mut buf.0);
        }
    })
    .cycle()
    .take(num_cpus::get())
    .map(std::thread::spawn)
    .collect::<Vec<_>>()
    .into_iter()
    .for_each(|handle| _ = handle.join());
}
