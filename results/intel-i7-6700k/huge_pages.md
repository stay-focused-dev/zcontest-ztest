# huge_pages - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/huge_pages 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/huge_pages > /dev/null
sieve: 298.3 ms, lookup: 221.2 ms, total: 519.4 ms

 Performance counter stats for 'target/release/huge_pages':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second     
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
               144      page-faults:u                    #    276.8 faults/sec  page_faults_per_second
            520.32 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized       
        13,868,945      branch-misses:u                  #      5.3 %  branch_miss_rate         (49.84%)
       260,329,761      branches:u                       #    500.3 M/sec  branch_frequency     (49.84%)
     2,152,668,238      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (50.04%)
     2,154,837,842      instructions:u                   #      1.0 instructions  insn_per_cycle  (50.16%)

       0.520659089 seconds time elapsed

       0.516269000 seconds user
       0.003322000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/huge_pages > /dev/null
sieve: 298.4 ms, lookup: 220.7 ms, total: 519.0 ms

 Performance counter stats for 'target/release/huge_pages':

       354,777,254      mem_load_retired.l1_hit:u                                               (66.52%)
       300,694,968      mem_load_retired.l1_miss:u                                              (66.52%)
       304,779,390      mem_load_retired.l2_hit:u                                               (66.52%)
           314,370      mem_load_retired.l2_miss:u                                              (66.93%)
           193,150      mem_load_retired.l3_hit:u                                               (66.96%)
           119,049      mem_load_retired.l3_miss:u                                              (66.55%)

       0.520133365 seconds time elapsed

       0.511730000 seconds user
       0.006654000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 354,777,254   | 300,694,968   | 54.1 %   |
| L2    | 304,779,390   |     314,370   | 99.9 %   |
| LLC   |     193,150   |     119,049   | 61.9 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/huge_pages > /dev/null'
Benchmark 1: taskset -c 0 target/release/huge_pages > /dev/null
  Time (mean ± σ):     520.9 ms ±   1.0 ms    [User: 516.3 ms, System: 4.2 ms]
  Range (min … max):   519.2 ms … 523.0 ms    20 runs
```
