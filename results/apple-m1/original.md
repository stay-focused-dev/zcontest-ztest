# original - apple M1

**OS:** MacOS 26.4 (25E246)
**Compiler:** Apple clang version 21.0.0 (clang-2100.0.123.102)

## How to compile

```sh
gcc -O3 -march=native -fwrapv -pipe -o original/solution original/main.c
```

## Correctness

```sh
$ original/solution 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'original/solution > /dev/null'                                                                                                                                                     00:18:53
Benchmark 1: original/solution > /dev/null
  Time (mean ± σ):     782.6 ms ±  12.5 ms    [User: 765.3 ms, System: 13.1 ms]
  Range (min … max):   759.2 ms … 812.4 ms    20 runs
```
