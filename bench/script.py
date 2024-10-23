import numpy as np
import boost_histogram as bh

import time

class Timer:
    def __enter__(self):
        self.start = time.perf_counter()
        return self

    def __exit__(self, *args):
        self.end = time.perf_counter()
        self.elapsed_ms = (self.end - self.start) * 1000

bins = (100, 100)
ranges = ((-3, 3), (-3, 3))
bins = np.asarray(bins).astype(np.int64)
ranges = np.asarray(ranges).astype(np.float64)

edges = (
    np.linspace(*ranges[0, :], bins[0] + 1),
    np.linspace(*ranges[1, :], bins[1] + 1),
)

np.random.seed(42)
vals = np.random.normal(size=[2, 10_000_000]).astype(np.float32)

hist = bh.Histogram(
    bh.axis.Regular(bins[0], *ranges[0]), bh.axis.Regular(bins[1], *ranges[1])
)

with Timer() as t:
    hist.fill(*vals, threads=1)

print(f"Fill took: {t.elapsed_ms:.2f} ms")
