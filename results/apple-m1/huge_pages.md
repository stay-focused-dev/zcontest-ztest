# huge_pages - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/huge_pages 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/huge_pages > /dev/null'
Benchmark 1: target/release/huge_pages > /dev/null
  Time (mean ± σ):     331.0 ms ±   2.7 ms    [User: 318.3 ms, System: 10.3 ms]
  Range (min … max):   328.7 ms … 339.8 ms    20 runs
```
