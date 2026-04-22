# l1_cache_again - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/l1_cache_again 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/l1_cache_again > /dev/null'
Benchmark 1: target/release/l1_cache_again > /dev/null
  Time (mean ± σ):     349.3 ms ±   1.2 ms    [User: 338.6 ms, System: 8.6 ms]
  Range (min … max):   347.5 ms … 352.5 ms    20 runs
```
