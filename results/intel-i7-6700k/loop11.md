# loop11 - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/loop11 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/loop11 > /dev/null
sieve: 360.9 ms, lookup: 246.5 ms, total: 607.4 ms

 Performance counter stats for 'target/release/loop11':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second     
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            34,913      page-faults:u                    #  57122.2 faults/sec  page_faults_per_second
            611.20 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized       
        13,690,311      branch-misses:u                  #      5.3 %  branch_miss_rate         (49.93%)
       259,249,971      branches:u                       #    424.2 M/sec  branch_frequency     (49.94%)
     2,344,261,590      cpu-cycles:u                     #      3.8 GHz  cycles_frequency       (49.94%)
     2,178,944,588      instructions:u                   #      0.9 instructions  insn_per_cycle  (50.07%)

       0.611578626 seconds time elapsed

       0.580200000 seconds user
       0.030012000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/loop11 > /dev/null
sieve: 359.9 ms, lookup: 244.4 ms, total: 604.3 ms

 Performance counter stats for 'target/release/loop11':

       342,193,330      mem_load_retired.l1_hit:u                                               (66.44%)
       310,812,452      mem_load_retired.l1_miss:u                                              (66.45%)
       301,272,404      mem_load_retired.l2_hit:u                                               (66.45%)
         9,653,001      mem_load_retired.l2_miss:u                                              (66.72%)
         8,700,143      mem_load_retired.l3_hit:u                                               (67.10%)
               647      mem_load_retired.l3_miss:u                                              (66.83%)

       0.608420610 seconds time elapsed

       0.543690000 seconds user
       0.063376000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 342,193,330   | 310,812,452   | 52.4 %   |
| L2    | 301,272,404   |   9,653,001   | 96.9 %   |
| LLC   |   8,700,143   |         647   | 100.0 %  |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/loop11 > /dev/null'
Benchmark 1: taskset -c 0 target/release/loop11 > /dev/null
  Time (mean ± σ):     610.6 ms ±   0.8 ms    [User: 571.2 ms, System: 38.9 ms]
  Range (min … max):   609.2 ms … 612.3 ms    20 runs
```
