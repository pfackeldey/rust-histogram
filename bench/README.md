# Run benchmark

rust-histogram

```bash
cargo build --release
./target/release/hist_bench
```

boost-histogram

```bash
uv venv
source .venv/bin/activate
uv pip install boost-histogram

python script.py
```

## Benchmark results for filling 10M entries with this implementation:

- `VecHist` (fill 10M): 151.70ms
- `SparseHist` (fill 10M): >10s
- `HashMapHist` (fill 10M): 235.31ms
- `boost-histogram` (fill 10M): 37.17 ms


## References:

- https://iscinumpy.gitlab.io/post/histogram-speeds-in-python/
- https://boost-histogram.readthedocs.io/en/latest/notebooks/PerformanceComparison.html
