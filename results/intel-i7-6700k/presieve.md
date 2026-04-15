# presieve - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/presieve 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/presieve > /dev/null
sieve: 505.3 ms, lookup: 249.4 ms, total: 754.7 ms

 Performance counter stats for 'target/release/presieve':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second     
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,591      page-faults:u                    #  23183.3 faults/sec  page_faults_per_second
            758.78 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized       
        21,023,190      branch-misses:u                  #      1.3 %  branch_miss_rate         (49.95%)
     1,624,026,467      branches:u                       #   2140.3 M/sec  branch_frequency     (50.20%)
     3,040,697,629      cpu-cycles:u                     #      4.0 GHz  cycles_frequency       (50.20%)
     5,675,493,316      instructions:u                   #      1.9 instructions  insn_per_cycle  (50.05%)

       0.759339608 seconds time elapsed

       0.747410000 seconds user
       0.010013000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/presieve > /dev/null
sieve: 506.1 ms, lookup: 246.9 ms, total: 753.0 ms

 Performance counter stats for 'target/release/presieve':

       570,360,134      mem_load_retired.l1_hit:u                                               (66.71%)
       248,039,703      mem_load_retired.l1_miss:u                                              (66.71%)
       244,582,312      mem_load_retired.l2_hit:u                                               (66.70%)
         5,026,318      mem_load_retired.l2_miss:u                                              (66.70%)
         5,379,884      mem_load_retired.l3_hit:u                                               (66.59%)
               150      mem_load_retired.l3_miss:u                                              (66.59%)

       0.757218943 seconds time elapsed

       0.735438000 seconds user
       0.019973000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 570,360,134   | 248,039,703   | 69.7 %   |
| L2    | 244,582,312   |   5,026,318   | 98.0 %   |
| LLC   |   5,379,884   |         150   | 100.0 %  |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/presieve > /dev/null'
Benchmark 1: taskset -c 0 target/release/presieve > /dev/null
  Time (mean ± σ):     758.4 ms ±   0.8 ms    [User: 734.3 ms, System: 23.3 ms]
  Range (min … max):   757.2 ms … 759.7 ms    20 runs
```
