# segmented_wheel235_div - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/segmented_wheel235_div 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/segmented_wheel235_div > /dev/null
sieve: 740.9 ms, lookup: 448.0 ms, total: 1188.9 ms

 Performance counter stats for 'target/release/segmented_wheel235_div':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,606      page-faults:u                    #  14762.5 faults/sec  page_faults_per_second
          1,192.62 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
         8,302,177      branch-misses:u                  #      0.4 %  branch_miss_rate         (49.81%)
     2,115,798,059      branches:u                       #   1774.1 M/sec  branch_frequency     (50.06%)
     4,866,145,813      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (50.20%)
    11,548,456,260      instructions:u                   #      2.4 instructions  insn_per_cycle  (50.19%)

       1.193225532 seconds time elapsed

       1.163986000 seconds user
       0.026600000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/segmented_wheel235_div > /dev/null
sieve: 739.6 ms, lookup: 450.0 ms, total: 1189.6 ms

 Performance counter stats for 'target/release/segmented_wheel235_div':

     2,222,987,305      mem_load_retired.l1_hit:u                                               (66.51%)
       297,205,458      mem_load_retired.l1_miss:u                                              (66.76%)
       278,192,843      mem_load_retired.l2_hit:u                                               (66.81%)
        18,013,595      mem_load_retired.l2_miss:u                                              (66.81%)
         3,881,545      mem_load_retired.l3_hit:u                                               (66.68%)
        14,125,204      mem_load_retired.l3_miss:u                                              (66.43%)

       1.193574002 seconds time elapsed

       1.164538000 seconds user
       0.026610000 seconds sys
```

| level | hits            | misses          | hit rate |
|-------|-----------------|-----------------|----------|
| L1    | 2,222,987,305   | 297,205,458     | 88.2 %   |
| L2    | 278,192,843     | 18,013,595      | 93.9 %   |
| LLC   | 3,881,545       | 14,125,204      | 21.5 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/segmented_wheel235_div > /dev/null'

Benchmark 1: taskset -c 0 target/release/segmented_wheel235_div > /dev/null
  Time (mean ± σ):      1.194 s ±  0.002 s    [User: 1.166 s, System: 0.027 s]
  Range (min … max):    1.192 s …  1.202 s    20 runs
```
