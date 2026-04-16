# smart_presieve - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/smart_presieve 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/smart_presieve > /dev/null'
Benchmark 1: target/release/smart_presieve > /dev/null
  Time (mean ± σ):     392.2 ms ±   2.3 ms    [User: 377.4 ms, System: 12.3 ms]
  Range (min … max):   389.7 ms … 399.7 ms    20 runs
```
