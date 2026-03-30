# segmented_wheel235 - apple M1

**OS:** MacOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/segmented_wheel235 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/segmented_wheel235 > /dev/null'                                                                                                                                     00:17:26
Benchmark 1: target/release/segmented_wheel235 > /dev/null
  Time (mean ± σ):     799.8 ms ±   9.7 ms    [User: 783.7 ms, System: 12.3 ms]
  Range (min … max):   788.8 ms … 828.3 ms    20 runs
```
