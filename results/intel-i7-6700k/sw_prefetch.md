# sw_prefetch - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/sw_prefetch 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/sw_prefetch > /dev/null
sieve: 568.0 ms, lookup: 247.3 ms, total: 815.3 ms

 Performance counter stats for 'target/release/sw_prefetch':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,609      page-faults:u                    #  21517.9 faults/sec  page_faults_per_second
            818.34 msec task-clock:u                     #      0.9 CPUs  CPUs_utilized
         8,361,679      branch-misses:u                  #      0.4 %  branch_miss_rate         (49.59%)
     1,908,880,165      branches:u                       #   2332.6 M/sec  branch_frequency     (49.94%)
     3,315,365,191      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (50.30%)
     6,738,319,414      instructions:u                   #      2.0 instructions  insn_per_cycle  (50.41%)

       0.819136840 seconds time elapsed

       0.796935000 seconds user
       0.020004000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/sw_prefetch > /dev/null
sieve: 566.6 ms, lookup: 245.8 ms, total: 812.4 ms

 Performance counter stats for 'target/release/sw_prefetch':

       557,072,542      mem_load_retired.l1_hit:u                                               (66.29%)
       298,926,382      mem_load_retired.l1_miss:u                                              (66.67%)
       298,550,476      mem_load_retired.l2_hit:u                                               (66.91%)
         1,122,638      mem_load_retired.l2_miss:u                                              (66.90%)
         1,015,294      mem_load_retired.l3_hit:u                                               (66.80%)
               234      mem_load_retired.l3_miss:u                                              (66.43%)

       0.816336916 seconds time elapsed

       0.783990000 seconds user
       0.030030000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 569,866,499   | 294,022,051   | 66.0 %   |
| L2    | 292,169,487   |   3,094,171   | 99.0 %   |
| LLC   |   3,929,744   |           315 | 100.0 %  |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/sw_prefetch > /dev/null'
Benchmark 1: taskset -c 0 target/release/sw_prefetch > /dev/null
  Time (mean ± σ):     821.1 ms ±   0.9 ms    [User: 794.8 ms, System: 24.9 ms]
  Range (min … max):   818.6 ms … 822.5 ms    20 runs
```
