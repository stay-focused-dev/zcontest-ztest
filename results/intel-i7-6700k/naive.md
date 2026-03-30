# naive - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/naive 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/naive > /dev/null
sieve: 11669.0 ms, lookup: 700.2 ms, total: 12369.2 ms

 Performance counter stats for 'target/release/naive':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
           524,378      page-faults:u                    #  42040.3 faults/sec  page_faults_per_second
         12,473.24 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
       115,568,400      branch-misses:u                  #      1.7 %  branch_miss_rate         (50.01%)
     6,909,888,149      branches:u                       #    554.0 M/sec  branch_frequency     (50.01%)
    48,757,434,892      cpu-cycles:u                     #      3.9 GHz  cycles_frequency       (50.01%)
    28,659,943,157      instructions:u                   #      0.6 instructions  insn_per_cycle  (49.99%)

      12.476025096 seconds time elapsed

      11.710518000 seconds user
       0.727783000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/naive > /dev/null

sieve: 11644.3 ms, lookup: 697.7 ms, total: 12342.0 ms

 Performance counter stats for 'target/release/naive':

     1,177,677,703      mem_load_retired.l1_hit:u                                               (66.65%)
        23,100,115      mem_load_retired.l1_miss:u                                              (66.65%)
             4,774      mem_load_retired.l2_hit:u                                               (66.67%)
        23,270,913      mem_load_retired.l2_miss:u                                              (66.69%)
            18,132      mem_load_retired.l3_hit:u                                               (66.68%)
        23,103,794      mem_load_retired.l3_miss:u                                              (66.66%)

      12.448415983 seconds time elapsed

      11.762799000 seconds user
       0.651469000 seconds sys
```

| level | hits              | misses         | hit rate |
|-------|-------------------|----------------|----------|
| L1    | 1,177,677,703     | 23,100,115     | 98.1 %   |
| L2    | 4,774             | 23,270,913     | 0.02 %   |
| LLC   | 18,132            | 23,103,794     | 0.08 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/naive > /dev/null'

Benchmark 1: taskset -c 0 target/release/naive > /dev/null
  Time (mean ± σ):     12.400 s ±  0.006 s    [User: 11.716 s, System: 0.669 s]
  Range (min … max):   12.391 s … 12.413 s    20 runs
```
