# segmented_wheel235_div - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/segmented_wheel235_div 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/segmented_wheel235_div > /dev/null'
Benchmark 1: target/release/segmented_wheel235_div > /dev/null
  Time (mean ± σ):     702.7 ms ±   8.1 ms    [User: 686.1 ms, System: 13.5 ms]
  Range (min … max):   692.5 ms … 721.4 ms    20 runs
```
