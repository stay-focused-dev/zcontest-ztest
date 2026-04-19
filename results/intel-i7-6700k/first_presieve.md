# first_presieve - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/first_presieve 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/first_presieve > /dev/null
sieve: 300.3 ms, lookup: 220.9 ms, total: 521.2 ms

 Performance counter stats for 'target/release/first_presieve':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second     
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
               144      page-faults:u                    #    275.8 faults/sec  page_faults_per_second
            522.20 msec task-clock:u                     #      0.9 CPUs  CPUs_utilized       
        13,924,019      branch-misses:u                  #      5.2 %  branch_miss_rate         (50.03%)
       265,136,226      branches:u                       #    507.7 M/sec  branch_frequency     (50.03%)
     2,154,991,692      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (50.03%)
     2,158,589,486      instructions:u                   #      1.0 instructions  insn_per_cycle  (49.97%)

       0.522641718 seconds time elapsed

       0.518157000 seconds user
       0.003336000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/first_presieve > /dev/null
sieve: 299.3 ms, lookup: 220.3 ms, total: 519.6 ms

 Performance counter stats for 'target/release/first_presieve':

       378,167,748      mem_load_retired.l1_hit:u                                               (66.56%)
       304,121,876      mem_load_retired.l1_miss:u                                              (66.56%)
       307,291,833      mem_load_retired.l2_hit:u                                               (66.56%)
           240,380      mem_load_retired.l2_miss:u                                              (66.62%)
           241,806      mem_load_retired.l3_hit:u                                               (66.88%)
               713      mem_load_retired.l3_miss:u                                              (66.82%)

       0.520705945 seconds time elapsed

       0.516361000 seconds user
       0.003326000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 378,167,748   | 304,121,876   | 55.4 %   |
| L2    | 307,291,833   |     240,380   | 99.9 %   |
| LLC   |     241,806   |         713   | 99.7 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/first_presieve > /dev/null'
Benchmark 1: taskset -c 0 target/release/first_presieve > /dev/null
  Time (mean ± σ):     520.7 ms ±   0.6 ms    [User: 515.9 ms, System: 4.4 ms]
  Range (min … max):   519.2 ms … 521.5 ms    20 runs
```
