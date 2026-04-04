# unrolled - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/unrolled 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/unrolled > /dev/null
sieve: 574.1 ms, lookup: 447.3 ms, total: 1021.4 ms

 Performance counter stats for 'target/release/unrolled':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,608      page-faults:u                    #  17187.4 faults/sec  page_faults_per_second
          1,024.47 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
         8,312,774      branch-misses:u                  #      0.4 %  branch_miss_rate         (49.97%)
     1,904,783,297      branches:u                       #   1859.3 M/sec  branch_frequency     (49.98%)
     4,172,594,786      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (50.00%)
     6,595,805,313      instructions:u                   #      1.6 instructions  insn_per_cycle  (50.03%)

       1.025645263 seconds time elapsed

       1.002121000 seconds user
       0.019982000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/unrolled > /dev/null
sieve: 573.5 ms, lookup: 447.8 ms, total: 1021.2 ms

 Performance counter stats for 'target/release/unrolled':

       646,000,908      mem_load_retired.l1_hit:u                                               (66.65%)
       261,923,365      mem_load_retired.l1_miss:u                                              (66.65%)
       245,915,000      mem_load_retired.l2_hit:u                                               (66.65%)
        17,249,216      mem_load_retired.l2_miss:u                                              (66.65%)
         3,225,341      mem_load_retired.l3_hit:u                                               (66.71%)
        14,039,594      mem_load_retired.l3_miss:u                                              (66.71%)

       1.025724609 seconds time elapsed

       1.006127000 seconds user
       0.016657000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 646,000,908   | 261,923,365   | 71.1 %   |
| L2    | 245,915,000   | 17,249,216    | 93.4 %   |
| LLC   | 3,225,341     | 14,039,594    | 18.7 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/unrolled > /dev/null'

Benchmark 1: taskset -c 0 target/release/unrolled > /dev/null
  Time (mean ± σ):      1.025 s ±  0.001 s    [User: 1.001 s, System: 0.023 s]
  Range (min … max):    1.024 s …  1.026 s    20 runs
```
