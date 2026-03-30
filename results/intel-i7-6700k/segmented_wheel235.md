# segmented_wheel235 - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/segmented_wheel235 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/segmented_wheel235 > /dev/null
sieve: 1005.2 ms, lookup: 428.8 ms, total: 1434.1 ms

 Performance counter stats for 'target/release/segmented_wheel235':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,606      page-faults:u                    #  12244.9 faults/sec  page_faults_per_second
          1,437.83 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
        15,463,598      branch-misses:u                  #      0.7 %  branch_miss_rate         (49.93%)
     2,290,178,448      branches:u                       #   1592.8 M/sec  branch_frequency     (49.94%)
     5,916,266,273      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (49.93%)
    12,345,354,908      instructions:u                   #      2.1 instructions  insn_per_cycle  (50.07%)

       1.438446630 seconds time elapsed

       1.405278000 seconds user
       0.029900000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/segmented_wheel235 > /dev/null
sieve: 1004.5 ms, lookup: 428.6 ms, total: 1433.1 ms

 Performance counter stats for 'target/release/segmented_wheel235':

     2,485,674,774      mem_load_retired.l1_hit:u                                               (66.60%)
       285,386,390      mem_load_retired.l1_miss:u                                              (66.60%)
       270,956,335      mem_load_retired.l2_hit:u                                               (66.60%)
        15,478,419      mem_load_retired.l2_miss:u                                              (66.61%)
         3,874,064      mem_load_retired.l3_hit:u                                               (66.79%)
        10,758,623      mem_load_retired.l3_miss:u                                              (66.78%)

       1.437654515 seconds time elapsed

       1.407935000 seconds user
       0.026630000 seconds sys
```

| level | hits            | misses          | hit rate |
|-------|-----------------|-----------------|----------|
| L1    | 2,485,674,774   | 285,386,390     | 89.7 %   |
| L2    | 270,956,335     | 15,478,419      | 94.6 %   |
| LLC   | 3,874,064       | 10,758,623      | 26.5 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/segmented_wheel235 > /dev/null'

Benchmark 1: taskset -c 0 target/release/segmented_wheel235 > /dev/null
  Time (mean ± σ):      1.434 s ±  0.001 s    [User: 1.412 s, System: 0.021 s]
  Range (min … max):    1.432 s …  1.436 s    20 runs
```
