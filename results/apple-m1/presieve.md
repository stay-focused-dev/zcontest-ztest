# presieve - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/presieve 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/presieve > /dev/null'
Benchmark 1: target/release/presieve > /dev/null
  Time (mean ± σ):     468.8 ms ±   1.9 ms    [User: 456.2 ms, System: 10.3 ms]
  Range (min … max):   466.7 ms … 475.0 ms    20 runs
```
