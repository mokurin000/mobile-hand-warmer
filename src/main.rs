#![feature(random)]

fn main() {
    std::iter::once(|| {
        loop {
            std::random::random::<u64>(..);
        }
    })
    .cycle()
    .take(num_cpus::get())
    .map(std::thread::spawn)
    .collect::<Vec<_>>()
    .into_iter()
    .for_each(|handle| _ = handle.join());
}
