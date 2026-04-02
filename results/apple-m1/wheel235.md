# wheel235 - Apple M1

**OS:** macOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/wheel235 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/wheel235 > /dev/null'
Benchmark 1: target/release/wheel235 > /dev/null
  Time (mean ± σ):      1.456 s ±  0.022 s    [User: 1.438 s, System: 0.013 s]
  Range (min … max):    1.436 s …  1.521 s    20 runs
```
