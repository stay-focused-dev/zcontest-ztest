# l1_cache - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/l1_cache 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/l1_cache > /dev/null
perf stat -- target/release/l1_cache > /dev/null
sieve: 546.0 ms, lookup: 245.1 ms, total: 791.1 ms

 Performance counter stats for 'target/release/l1_cache':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,597      page-faults:u                    #  22142.9 faults/sec  page_faults_per_second
            794.70 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
         7,171,757      branch-misses:u                  #      0.4 %  branch_miss_rate         (49.89%)
     1,894,649,868      branches:u                       #   2384.1 M/sec  branch_frequency     (50.18%)
     3,192,953,513      cpu-cycles:u                     #      4.0 GHz  cycles_frequency       (50.19%)
     6,769,972,265      instructions:u                   #      2.1 instructions  insn_per_cycle  (50.11%)

       0.795287468 seconds time elapsed

       0.773368000 seconds user
       0.019999000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/l1_cache > /dev/null
sieve: 547.1 ms, lookup: 245.0 ms, total: 792.1 ms

 Performance counter stats for 'target/release/l1_cache':

       921,667,572      mem_load_retired.l1_hit:u                                               (66.44%)
       123,945,654      mem_load_retired.l1_miss:u                                              (66.81%)
       111,023,689      mem_load_retired.l2_hit:u                                               (66.85%)
        13,615,059      mem_load_retired.l2_miss:u                                              (66.85%)
        13,341,894      mem_load_retired.l3_hit:u                                               (66.71%)
             1,041      mem_load_retired.l3_miss:u                                              (66.34%)

       0.796723841 seconds time elapsed

       0.778285000 seconds user
       0.016626000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 924,032,080   | 123,423,096   | 88.2 %   |
| L2    | 108,315,831   |  15,258,040   | 87.7 %   |
| LLC   |  15,375,788   |         988   | 100.0 %  |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/l1_cache > /dev/null'
Benchmark 1: taskset -c 0 target/release/l1_cache > /dev/null
  Time (mean ± σ):     799.6 ms ±   0.9 ms    [User: 775.1 ms, System: 23.7 ms]
  Range (min … max):   798.5 ms … 802.5 ms    20 runs
```
