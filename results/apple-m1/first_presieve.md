# first_presieve - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/first_presieve 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/first_presieve > /dev/null'
Benchmark 1: target/release/first_presieve > /dev/null
  Time (mean ± σ):     327.0 ms ±   2.1 ms    [User: 313.1 ms, System: 11.4 ms]
  Range (min … max):   324.0 ms … 330.3 ms    20 runs
```
