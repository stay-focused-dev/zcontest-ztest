# l1_cache - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/l1_cache 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/l1_cache > /dev/null'
Benchmark 1: target/release/l1_cache > /dev/null
  Time (mean ± σ):     525.4 ms ±   2.4 ms    [User: 498.0 ms, System: 25.3 ms]
  Range (min … max):   521.3 ms … 529.3 ms    20 runs
```
