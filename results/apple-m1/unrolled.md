# unrolled - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/unrolled 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/unrolled > /dev/null'
Benchmark 1: target/release/unrolled > /dev/null
  Time (mean ± σ):     521.7 ms ±   1.3 ms    [User: 509.4 ms, System: 10.1 ms]
  Range (min … max):   519.2 ms … 524.4 ms    20 runs
```
