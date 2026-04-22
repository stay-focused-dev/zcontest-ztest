# l1_cache_again - Intel i7-6700K

**OS:** Gentoo
**Kernel:** 6.19.10-gentoo-dist
**Compiler:** rustc 1.94.0 (4a4ef493e 2026-03-02)

## How to compile

```sh
cargo build --release
```

## Correctness

```sh
$ target/release/l1_cache_again 2>/dev/null | md5sum
b197ddca5540e6e611fec0945e3d5573  -
```

## perf stat

```sh
$ perf stat -- target/release/l1_cache_again > /dev/null
sieve: 222.7 ms, lookup: 224.3 ms, total: 447.0 ms

 Performance counter stats for 'target/release/l1_cache_again':

                 7      context-switches                 #     15.6 cs/sec  cs_per_second     
                 0      cpu-migrations                   #      0.0 migrations/sec  migrations_per_second
               156      page-faults                      #    348.5 faults/sec  page_faults_per_second
            447.62 msec task-clock                       #      0.9 CPUs  CPUs_utilized       
        17,491,994      branch-misses                    #      6.5 %  branch_miss_rate         (49.75%)
       266,193,644      branches                         #    594.7 M/sec  branch_frequency     (49.74%)
     1,882,446,579      cpu-cycles                       #      4.2 GHz  cycles_frequency       (50.16%)
     2,168,859,674      instructions                     #      1.2 instructions  insn_per_cycle  (50.25%)

       0.448109553 seconds time elapsed

       0.443707000 seconds user
       0.003329000 seconds sys
```

## Cache counters

```sh
$ perf stat -e mem_load_retired.l1_hit,mem_load_retired.l1_miss,mem_load_retired.l2_hit,mem_load_retired.l2_miss,mem_load_retired.l3_hit,mem_load_retired.l3_miss -- target/release/l1_cache_again > /dev/null
sieve: 223.4 ms, lookup: 223.4 ms, total: 446.8 ms

 Performance counter stats for 'target/release/l1_cache_again':

       549,674,794      mem_load_retired.l1_hit                                                 (66.52%)
       113,826,249      mem_load_retired.l1_miss                                                (66.51%)
       113,588,199      mem_load_retired.l2_hit                                                 (66.52%)
         1,819,534      mem_load_retired.l2_miss                                                (66.53%)
         1,783,870      mem_load_retired.l3_hit                                                 (66.97%)
            19,618      mem_load_retired.l3_miss                                                (66.96%)

       0.448448394 seconds time elapsed

       0.444010000 seconds user
       0.003334000 seconds sys
```

| level | hits          | misses        | hit rate |
|-------|---------------|---------------|----------|
| L1    | 549,674,794   | 113,826,249   | 82.8 %   |
| L2    | 113,588,199   |   1,819,534   | 98.4 %   |
| LLC   |   1,783,870   |      19,618   | 98.9 %   |

## hyperfine

```sh
$ hyperfine --warmup 3 --runs 20 'taskset -c 0 target/release/l1_cache_again > /dev/null'
Benchmark 1: taskset -c 0 target/release/l1_cache_again > /dev/null
  Time (mean ± σ):     448.8 ms ±   0.3 ms    [User: 443.3 ms, System: 5.0 ms]
  Range (min … max):   448.3 ms … 449.3 ms    20 runs
```
