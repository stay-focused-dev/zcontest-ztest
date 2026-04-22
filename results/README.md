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
| original solution        |  1.067 s ± 0.003 s  | 11.6×    |
| sw_prefetch              |  0.821 s ± 0.001 s  | 15.1×    |
| l1_cache                 |  0.800 s ± 0.001 s  | 15.5×    |
| presieve                 |  0.758 s ± 0.001 s  | 16.4×    |
| smart_presieve           |  0.634 s ± 0.001 s  | 19.6×    |
| loop11                   |  0.610 s ± 0.001 s  | 20.3×    |
| huge_pages               |  0.521 s ± 0.001 s  | 23.8×    |
| first_presieve           |  0.521 s ± 0.001 s  | 23.8×    |
| l1_cache_again           |  0.449 s ± 0.000 s  | 27.6×    |
| branch_opt               |  0.440 s ± 0.001 s  | 28.2×    |

### Instructions (perf stat)

| variant                  | instructions       | insn/cycle |
|--------------------------|--------------------|------------|
| naive                    | 28,659,943,157     | 0.6        |
| segmented                | 17,433,870,718     | 1.4        |
| wheel235                 |  5,373,709,937     | 0.4        |
| segmented_wheel235       | 12,345,354,908     | 2.1        |
| segmented_wheel235_div   | 11,548,456,260     | 2.4        |
| unrolled                 |  6,595,805,313     | 1.6        |
| original solution        |  6,298,807,674     | 1.5        |
| sw_prefetch              |  6,738,319,414     | 2.0        |
| l1_cache                 |  6,769,972,265     | 2.1        |
| presieve                 |  5,675,493,316     | 1.9        |
| smart_presieve           |  4,034,881,598     | 1.6        |
| loop11                   |  2,178,944,588     | 0.9        |
| huge_pages               |  2,154,837,842     | 1.0        |
| first_presieve           |  2,158,589,486     | 1.0        |
| l1_cache_again           |  2,168,859,674     | 1.2        |
| branch_opt               |  2,216,253,197     | 1.2        |

### Cache hit rates (mem_load_retired.*)

| variant                  | L1 hit  | L2 hit  | LLC hit | L3 miss     |
|--------------------------|---------|---------|---------|-------------|
| naive                    | 98.1 %  |  0.02 % |  0.08 % | 23,103,794  |
| segmented                | 88.3 %  |  4.8 %  |  0.25 % | 22,369,900  |
| wheel235                 | 76.5 %  |  6.7 %  | 18.0 %  | 260,331,764 |
| segmented_wheel235       | 89.7 %  | 94.6 %  | 26.5 %  | 10,758,623  |
| segmented_wheel235_div   | 88.2 %  | 93.9 %  | 21.5 %  | 14,125,204  |
| unrolled                 | 71.1 %  | 93.4 %  | 18.7 %  | 14,039,594  |
| original solution        | 60.9 %  | 96.7 %  |  6.3 %  |  9,880,688  |
| sw_prefetch              | 65.1 %  | 99.6 %  | 100.0 % |         234 |
| l1_cache                 | 88.2 %  | 87.7 %  | 100.0 % |         988 |
| presieve                 | 69.7 %  | 98.0 %  | 100.0 % |         150 |
| smart_presieve           | 57.9 %  | 95.7 %  | 100.0 % |         225 |
| loop11                   | 52.4 %  | 96.9 %  | 100.0 % |         647 |
| huge_pages               | 54.1 %  | 99.9 %  |  61.9 % |     119,049 |
| first_presieve           | 55.4 %  | 99.9 %  |  99.7 % |         713 |
| l1_cache_again           | 82.8 %  | 98.4 %  |  98.9 % |      19,618 |
| branch_opt               | 84.5 %  | 98.6 %  |  99.2 % |      12,660 |

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
| original solution        |  0.783 s ± 0.013 s  | 11.5×    |
| sw_prefetch              |  0.484 s ± 0.002 s  | 18.5×    |
| l1_cache                 |  0.525 s ± 0.002 s  | 17.1×    |
| presieve                 |  0.469 s ± 0.002 s  | 19.1×    |
| smart_presieve           |  0.392 s ± 0.002 s  | 22.9×    |
| loop11                   |  0.329 s ± 0.002 s  | 27.3×    |
| huge_pages               |  0.331 s ± 0.003 s  | 27.1×    |
| first_presieve           |  0.327 s ± 0.002 s  | 27.4×    |
| l1_cache_again           |  0.349 s ± 0.001 s  | 25.7×    |
| branch_opt               |  0.341 s ± 0.002 s  | 26.3×    |
