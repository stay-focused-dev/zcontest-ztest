# wheel235 - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/wheel235 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/wheel235 > /dev/null
sieve: 2673.0 ms, lookup: 426.1 ms, total: 3099.1 ms

 Performance counter stats for 'target/release/wheel235':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,588      page-faults:u                    #   5669.5 faults/sec  page_faults_per_second
          3,102.24 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
        10,090,179      branch-misses:u                  #      0.8 %  branch_miss_rate         (49.92%)
     1,271,930,887      branches:u                       #    410.0 M/sec  branch_frequency     (49.96%)
    12,889,120,735      cpu-cycles:u                     #      4.2 GHz  cycles_frequency       (50.06%)
     5,373,709,937      instructions:u                   #      0.4 instructions  insn_per_cycle  (50.08%)

       3.103208303 seconds time elapsed

       3.069258000 seconds user
       0.023264000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/wheel235 > /dev/null
sieve: 2811.2 ms, lookup: 427.4 ms, total: 3238.6 ms

 Performance counter stats for 'target/release/wheel235':

     1,106,329,139      mem_load_retired.l1_hit:u                                               (66.61%)
       340,347,418      mem_load_retired.l1_miss:u                                              (66.61%)
        22,712,764      mem_load_retired.l2_hit:u                                               (66.70%)
       317,001,041      mem_load_retired.l2_miss:u                                              (66.70%)
        57,178,995      mem_load_retired.l3_hit:u                                               (66.69%)
       260,331,764      mem_load_retired.l3_miss:u                                              (66.69%)

       3.243808130 seconds time elapsed

       3.195797000 seconds user
       0.036550000 seconds sys
```

| level | hits            | misses          | hit rate |
|-------|-----------------|-----------------|----------|
| L1    | 1,106,329,139   | 340,347,418     | 76.5 %   |
| L2    | 22,712,764      | 317,001,041     |  6.7 %   |
| LLC   | 57,178,995      | 260,331,764     | 18.0 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/wheel235 > /dev/null'

Benchmark 1: taskset -c 0 target/release/wheel235 > /dev/null
  Time (mean ± σ):      3.238 s ±  0.015 s    [User: 3.207 s, System: 0.026 s]
  Range (min … max):    3.195 s …  3.249 s    20 runs
```
