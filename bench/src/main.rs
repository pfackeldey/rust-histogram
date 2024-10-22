use hist::hist::Histogram;
use hist_axes::axis::Axis;
use hist_axes::uniform::Uniform;
use hist_dense::vechist::VecHist;
use hist_sparse::hashmaphist::HashMapHist;
use hist_sparse::sparsehist::SparseHist;
use hist_storages::StorageType;

use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    let ax1 = Uniform::new(100, -1.0, 1.0).unwrap();
    let ax2 = Uniform::new(100, -1.0, 1.0).unwrap();

    // Generate 10M random entries
    let mut entries = vec![];
    let mut weights = vec![];
    for _ in 0..10_000_000 {
        let idxs = vec![
            ax1.index(thread_rng().gen_range(-1.0..1.0) as f64),
            ax2.index(thread_rng().gen_range(-1.0..1.0) as f64),
        ];
        entries.push(idxs);
        weights.push(1.0);
    }

    // VecHist benchmark
    let axes = vec![
        Box::new(ax1.clone()) as Box<dyn Axis>,
        Box::new(ax2.clone()) as Box<dyn Axis>,
    ];
    let mut hist = VecHist::new(axes, StorageType::Double);

    let now = Instant::now();
    hist.fill_n(&entries, &weights).unwrap();
    let elapsed = now.elapsed();
    println!("VecHist (fill 10M): {:.2?}", elapsed);

    // SparseHist benchmark
    let axes = vec![
        Box::new(ax1.clone()) as Box<dyn Axis>,
        Box::new(ax2.clone()) as Box<dyn Axis>,
    ];
    let mut hist = SparseHist::new(axes, StorageType::Double);

    let now = Instant::now();
    hist.fill_n(&entries, &weights).unwrap();
    let elapsed = now.elapsed();
    println!("SparseHist (fill 10M): {:.2?}", elapsed);

    // HashMapHist benchmark
    let axes = vec![
        Box::new(ax1.clone()) as Box<dyn Axis>,
        Box::new(ax2.clone()) as Box<dyn Axis>,
    ];
    let mut hist = HashMapHist::new(axes, StorageType::Double);

    let now = Instant::now();
    hist.fill_n(&entries, &weights).unwrap();
    let elapsed = now.elapsed();
    println!("HashMapHist (fill 10M): {:.2?}", elapsed);
}
