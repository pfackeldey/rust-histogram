# rust-histogram

## Simple example:

(todo: expose axes and histograms better to public API)

```rust
use hist::hist::Histogram;
use hist_axes::axis::Axis;
use hist_axes::uniform::Uniform;
use hist_axes::variable::Variable;
// stores data in a Vec of axes dimensionality
use hist_dense::densehist::VecHist as Hist;

// stores data in two Vecs but only for filled (non-zero) bins:
// - Vec<usize>: for the axis indices
// - Vec<f64>: for the values
// use hist_sparse::sparsehist::SparseHist as Hist;

// stores data in a HashMap<usize, f64> for filled (non-zero) bins
// use hist_sparse::hashmaphist::HashMapHist as Hist;

let uniform = Uniform::new("axis1".to_string(), 0.0, 10.0, 10);
let variable = Variable::new("axis2".to_string(), vec![0.0, 1.0, 2.0, 3.5, 4.1]);

let axis1 = Box::new(uniform) as Box<dyn Axis>;
let axis2 = Box::new(variable) as Box<dyn Axis>;

let mut hist = Hist::new(vec![axis1, axis2]);

// fill the histogram
hist.fill(&[1.0, 1.0], 1.0);

println!("{:?}", hist);
```


## Run tests

```bash
cargo test --workspace
```
