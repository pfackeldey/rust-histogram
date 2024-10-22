# rust-histogram

## Simple example:

(todo: expose axes and histograms better to public API)

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
