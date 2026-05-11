# mobile-hand-warmer

Rapidly drives all CPU cores to 100% utilization.

This program spawns `N` threads (`N` = number of logical CPU cores) and continuously generates pseudo-random numbers in an infinite loop.

## Warning

This may negatively impact your phone's battery lifespan.

If the device's thermal protection fails, excessive overheating could potentially cause physical damage.

## Build

Inside Termux:

```bash
export RUSTC_BOOTSTRAP=1
cargo build --release
```
