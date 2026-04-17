# loop11 - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/loop11 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/loop11 > /dev/null'
Benchmark 1: target/release/loop11 > /dev/null
  Time (mean ± σ):     328.8 ms ±   2.4 ms    [User: 316.7 ms, System: 10.0 ms]
  Range (min … max):   326.2 ms … 333.8 ms    20 runs
```
