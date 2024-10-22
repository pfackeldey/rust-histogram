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

```rust
use hist::hist::Histogram;
use hist_axes::category::Category;
use hist_axes::uniform::Uniform;
use hist_storages::{Storage, StorageType};
use hist_dense::vechist::VecHist as Hist;
// use hist_sparse::sparsehist::SparseHist as Hist;
// use hist_sparse::hashmap::HashMapHist as Hist;

let uniform = Uniform::new(10, 0.0, 10.0).unwrap();
let cat = Category::new(vec![
    "A".to_string(),
    "B".to_string(),
    "C".to_string(),
    "D".to_string(),
    "E".to_string(),
]).unwrap();

let axes = vec![
    Box::new(uniform.clone()) as Box<dyn Axis>,
    Box::new(cat.clone()) as Box<dyn Axis>,
];
let mut hist = Hist::new(axes, StorageType::Double);
assert_eq!(hist.get_axes().len(), 2);
assert_eq!(hist.num_bins(false), 50); // without flow
assert_eq!(hist.num_bins(true), 72); // with flow

// Fill the histogram
let where2fill = vec![uniform.index(0.0), cat.index("A".to_string())];
hist.fill(where2fill, 1.0).unwrap();

assert_eq!(hist.num_bins(false), 50);
assert_eq!(hist.num_bins(true), 72);
assert_eq!(hist.get_bin(0), Storage::Double(0.0));

// Check the bin
let idxs = vec![uniform.index(0.0), cat.index("A".to_string())];
assert_eq!(
    hist.get_bin(hist.stride_index(idxs).unwrap()),
    Storage::Double(1.0)
);

println!("{:?}", hist);
```


## Run tests

```bash
cargo test --workspace
```
