# segmented - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/segmented 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/segmented > /dev/null
sieve: 2952.2 ms, lookup: 701.0 ms, total: 3653.2 ms

 Performance counter stats for 'target/release/segmented':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
           524,401      page-faults:u                    # 139618.8 faults/sec  page_faults_per_second
          3,755.95 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
        33,556,872      branch-misses:u                  #      0.6 %  branch_miss_rate         (49.93%)
     5,291,669,962      branches:u                       #   1408.9 M/sec  branch_frequency     (49.93%)
    12,168,720,673      cpu-cycles:u                     #      3.2 GHz  cycles_frequency       (50.02%)
    17,433,870,718      instructions:u                   #      1.4 instructions  insn_per_cycle  (50.07%)

       3.757032386 seconds time elapsed

       3.068530000 seconds user
       0.678552000 seconds sys

```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/segmented > /dev/null
sieve: 2934.4 ms, lookup: 699.7 ms, total: 3634.1 ms

 Performance counter stats for 'target/release/segmented':

       177,801,660      mem_load_retired.l1_hit:u                                               (66.62%)
        23,475,624      mem_load_retired.l1_miss:u                                              (66.62%)
         1,138,480      mem_load_retired.l2_hit:u                                               (66.67%)
        22,349,148      mem_load_retired.l2_miss:u                                              (66.75%)
            56,092      mem_load_retired.l3_hit:u                                               (66.71%)
        22,369,900      mem_load_retired.l3_miss:u                                              (66.63%)

       3.738996696 seconds time elapsed

       3.080253000 seconds user
       0.648970000 seconds sys
```

| level | hits          | misses         | hit rate |
|-------|---------------|----------------|----------|
| L1    | 177,801,660   | 23,475,624     | 88.3 %   |
| L2    | 1,138,480     | 22,349,148     |  4.8 %   |
| LLC   | 56,092        | 22,369,900     |  0.25 %  |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/segmented > /dev/null'

Benchmark 1: taskset -c 0 target/release/segmented > /dev/null
  Time (mean ± σ):      3.735 s ±  0.002 s    [User: 3.068 s, System: 0.662 s]
  Range (min … max):    3.731 s …  3.740 s    20 runs
```
