# Prime sieve benchmarks

Comparison of prime sieve implementations in Rust, benchmarked against the original C solution.

## Problem

Given a sequence defined by:
```
a(0) = 1
a(n) = (a(n-1) + 1234567890) mod 2147483648
```

Output `1` if `a(n)` is prime, `0` otherwise, for n = 1..33333333.

## Implementations

| variant            | language | description                                      |
|--------------------|----------|--------------------------------------------------|
| naive              | Rust     | basic sieve of Eratosthenes                      |
| segmented          | Rust     | segmented sieve, cache-friendly chunks           |
| wheel235           | Rust     | wheel factorization mod 30, 8 bits per 30 nums   |
| segmented_wheel235 | Rust     | wheel mod 30 + segmented, best of both           |
| original solution  | C        | reference implementation                         |

## How to build

**Rust:**
```sh
cargo build --release
```

**C (original solution):**
```sh
gcc -O3 -march=native -fwrapv -pipe -o original/solution original/main.c
```

## Results

See [results/README.md](results/README.md) for full benchmark tables.

## Tested on

| machine    | OS                         | compiler                                        |
|------------|----------------------------|-------------------------------------------------|
| i7-6700K   | Gentoo, kernel 6.19.10     | rustc 1.94.0 / gcc 15.2.1                       |
| Apple M1   | macOS 26.4                 | rustc 1.92.0 / Apple clang 21.0.0               |
