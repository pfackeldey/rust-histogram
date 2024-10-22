# rust-histogram

A Rust library for histograms with different storage types and axis types.
Highly inspired by the [`boost-histogram`](https://github.com/scikit-hep/boost-histogram) library.
In addition, it provides sparse histogram implementations for very high-dimensional histograms.

## Available Components

- StorageTypes:
  - `Double`: stores `sumw` as `f64`
  - `Int`: stores `sumw` as `i64`
  - `Weight`: stores `sumw` and `sumw2` as (`f64`, `f64`) (tuple)
- Axis (note: `boost-histogram` like `growth` is not supported!):
  - `Uniform`: constructs a uniform axis with `n` bins between `start` and `stop`.
  - `Variable`: constructs a variable axis with `edges` as bin edges.
  - `Category`: constructs a categorical axis with `String` as bin labels.
  - `Integer`: constructs a categorical axis with `i64` as bin labels.
- Hist:
  - `VecHist`: stores the histogram bins in a `Vec<StorageType>` (dense).
  - `SparseHist`: stores the histogram contents and indices in a `Vec<StorageType>` respectively (sparse). Here only the filled bins are stored.
  - `HashMapHist`: stores the histogram in a `HashMap<usize, StorageType>` (sparse). Here only the filled bins are stored.

## Example:

See `bench/src/main.rs` for an example.


## Run tests

```bash
cargo test --workspace
```
