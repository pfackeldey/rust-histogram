# Run benchmark

```bash
cargo build --release
./target/release/hist_bench
```

## Benchmark results for filling 10M entries with this implementation:
- `VecHist` (fill 10M): 55.35ms
- `SparseHist` (fill 10M): 12.71s
- `HashMapHist` (fill 10M): 105.98ms


## References:

- https://iscinumpy.gitlab.io/post/histogram-speeds-in-python/
- https://boost-histogram.readthedocs.io/en/latest/notebooks/PerformanceComparison.html
