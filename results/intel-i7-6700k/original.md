# original solution - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** gcc version 15.2.1 20260214 (Gentoo 15.2.1_p20260214 p5)

## How to compile

```sh
gcc -O3 -march=native -fwrapv -pipe -o original/solution original/main.c
```

## Correctness

```sh
$ original/solution | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- original/solution > /dev/null

 Performance counter stats for 'original/solution':

                 0      context-switches:u               #      0.0 cs/sec  cs_per_second
                 0      cpu-migrations:u                 #      0.0 migrations/sec  migrations_per_second
            17,628      page-faults:u                    #  16547.3 faults/sec  page_faults_per_second
          1,065.31 msec task-clock:u                     #      1.0 CPUs  CPUs_utilized
        20,137,423      branch-misses:u                  #      2.1 %  branch_miss_rate         (49.87%)
       973,855,214      branches:u                       #    914.2 M/sec  branch_frequency     (50.15%)
     4,337,673,686      cpu-cycles:u                     #      4.1 GHz  cycles_frequency       (50.15%)
     6,298,807,674      instructions:u                   #      1.5 instructions  insn_per_cycle  (50.13%)

       1.069017595 seconds time elapsed

       1.036583000 seconds user
       0.030005000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- original/solution > /dev/null

 Performance counter stats for 'original/solution':

       478,567,933      mem_load_retired.l1_hit:u                                               (66.68%)
       307,552,357      mem_load_retired.l1_miss:u                                              (66.79%)
       301,533,926      mem_load_retired.l2_hit:u                                               (66.80%)
        10,446,330      mem_load_retired.l2_miss:u                                              (66.80%)
           667,151      mem_load_retired.l3_hit:u                                               (66.52%)
         9,880,688      mem_load_retired.l3_miss:u                                              (66.41%)

       1.069907708 seconds time elapsed

       1.047682000 seconds user
       0.019957000 seconds sys
```

| level | hits          | misses          | hit rate |
|-------|---------------|-----------------|----------|
| L1    | 478,567,933   | 307,552,357     | 60.9 %   |
| L2    | 301,533,926   | 10,446,330      | 96.7 %   |
| LLC   | 667,151       | 9,880,688       |  6.3 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 original/solution > /dev/null'

Benchmark 1: taskset -c 0 original/solution > /dev/null
  Time (mean ± σ):      1.067 s ±  0.003 s    [User: 1.036 s, System: 0.029 s]
  Range (min … max):    1.063 s …  1.074 s    20 runs
```
