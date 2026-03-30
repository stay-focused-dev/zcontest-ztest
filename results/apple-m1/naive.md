# naive - apple M1

**OS:** MacOS 26.4 (25E246)
**Compiler:** rustc 1.92.0 (ded5c06cf 2025-12-08)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/naive 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'target/release/naive > /dev/null'                                                                                                                                                  00:09:18
Benchmark 1: target/release/naive > /dev/null
  Time (mean ± σ):      8.974 s ±  0.181 s    [User: 8.731 s, System: 0.216 s]
  Range (min … max):    8.814 s …  9.469 s    20 runs
```
