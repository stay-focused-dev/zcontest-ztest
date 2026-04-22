# branch_opt - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/branch_opt 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/branch_opt > /dev/null'
Benchmark 1: target/release/branch_opt > /dev/null
  Time (mean ± σ):     341.4 ms ±   1.6 ms    [User: 328.5 ms, System: 10.5 ms]
  Range (min … max):   339.2 ms … 345.8 ms    20 runs
```
