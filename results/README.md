# Benchmark summary

## Intel i7-6700K

### Time (hyperfine, mean ± σ)

| variant                  | time                | vs naive |
|--------------------------|---------------------|----------|
| naive                    | 12.400 s ± 0.006 s  | —        |
| segmented                |  3.735 s ± 0.002 s  |  3.3×    |
| wheel235                 |  3.238 s ± 0.015 s  |  3.8×    |
| segmented_wheel235       |  1.434 s ± 0.001 s  |  8.6×    |
| segmented_wheel235_div   |  1.194 s ± 0.002 s  | 10.4×    |
| unrolled                 |  1.025 s ± 0.001 s  | 12.1×    |
| sw_prefetch              |  0.821 s ± 0.001 s  | 15.1×    |
| original solution        |  1.067 s ± 0.003 s  | 11.6×    |

### Instructions (perf stat)

| variant                  | instructions       | insn/cycle |
|--------------------------|--------------------|------------|
| naive                    | 28,659,943,157     | 0.6        |
| segmented                | 17,433,870,718     | 1.4        |
| wheel235                 |  5,373,709,937     | 0.4        |
| segmented_wheel235       | 12,345,354,908     | 2.1        |
| segmented_wheel235_div   | 11,548,456,260     | 2.4        |
| unrolled                 |  6,595,805,313     | 1.6        |
| sw_prefetch              |  6,738,319,414     | 2.0        |
| original solution        |  6,298,807,674     | 1.5        |

### Cache hit rates (mem_load_retired.*)

| variant                  | L1 hit  | L2 hit  | LLC hit | L3 miss     |
|--------------------------|---------|---------|---------|-------------|
| naive                    | 98.1 %  |  0.02 % |  0.08 % | 23,103,794  |
| segmented                | 88.3 %  |  4.8 %  |  0.25 % | 22,369,900  |
| wheel235                 | 76.5 %  |  6.7 %  | 18.0 %  | 260,331,764 |
| segmented_wheel235       | 89.7 %  | 94.6 %  | 26.5 %  | 10,758,623  |
| segmented_wheel235_div   | 88.2 %  | 93.9 %  | 21.5 %  | 14,125,204  |
| unrolled                 | 71.1 %  | 93.4 %  | 18.7 %  | 14,039,594  |
| sw_prefetch              | 65.1 %  | 99.6 %  | 100.0 % |         234 |
| original solution        | 60.9 %  | 96.7 %  |  6.3 %  |  9,880,688  |

## Apple M1

### Time (hyperfine, mean ± σ)

| variant                  | time                | vs naive |
|--------------------------|---------------------|----------|
| naive                    |  8.974 s ± 0.181 s  | —        |
| segmented                |  1.687 s ± 0.037 s  |  5.3×    |
| wheel235                 |  1.456 s ± 0.022 s  |  6.2×    |
| segmented_wheel235       |  0.800 s ± 0.010 s  | 11.2×    |
| segmented_wheel235_div   |  0.703 s ± 0.008 s  | 12.8×    |
| unrolled                 |  0.522 s ± 0.001 s  | 17.2×    |
| sw_prefetch              |  0.484 s ± 0.002 s  | 18.5×    |
| original solution        |  0.783 s ± 0.013 s  | 11.5×    |
