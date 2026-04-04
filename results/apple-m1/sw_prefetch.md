# sw_prefetch - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/sw_prefetch 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/sw_prefetch > /dev/null'
Benchmark 1: target/release/sw_prefetch > /dev/null
  Time (mean ± σ):     483.8 ms ±   2.1 ms    [User: 470.3 ms, System: 11.2 ms]
  Range (min … max):   480.3 ms … 487.1 ms    20 runs
```
