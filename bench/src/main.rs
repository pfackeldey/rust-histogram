use hist::hist::Histogram;
use hist_axes::axis::Axis;
use hist_axes::uniform::Uniform;
use hist_dense::vechist::VecHist;
use hist_sparse::hashmaphist::HashMapHist;
use hist_sparse::sparsehist::SparseHist;
use hist_storages::StorageType;

use rand::{thread_rng, Rng};
use std::time::Instant;

fn fill_n(
    hist: &mut dyn Histogram,
    ax1: &Uniform,
    ax2: &Uniform,
    ax1_entries: &Vec<f32>,
    ax2_entries: &Vec<f32>,
    weights: &Vec<f32>,
) {
    let num_entries = ax1_entries.len();
    for i in 0..num_entries {
        let idx1 = ax1.index(ax1_entries[i]);
        let idx2 = ax2.index(ax2_entries[i]);
        let indices = vec![idx1, idx2];
        hist.fill(&indices, weights[i]).unwrap();
    }
}

fn main() {
    let ax1 = Uniform::new(100, -1.0, 1.0).unwrap();
    let ax2 = Uniform::new(100, -1.0, 1.0).unwrap();

    // Generate 10M random entries
    let mut ax1entries = vec![];
    let mut ax2entries = vec![];
    let mut weights = vec![];
    for _ in 0..10_000_000 {
        ax1entries.push(thread_rng().gen_range(-1.0..1.0) as f32);
        ax2entries.push(thread_rng().gen_range(-1.0..1.0) as f32);
        weights.push(1.0);
    }

    // VecHist benchmark
    let axes = vec![
        Box::new(ax1.clone()) as Box<dyn Axis>,
        Box::new(ax2.clone()) as Box<dyn Axis>,
    ];
    let mut hist = VecHist::new(axes, StorageType::Double);

    let now = Instant::now();
    fill_n(&mut hist, &ax1, &ax2, &ax1entries, &ax2entries, &weights);
    let elapsed = now.elapsed();
    println!("VecHist (fill 10M): {:.2?}", elapsed);

    // SparseHist benchmark
    let axes = vec![
        Box::new(ax1.clone()) as Box<dyn Axis>,
        Box::new(ax2.clone()) as Box<dyn Axis>,
    ];
    let mut hist = SparseHist::new(axes, StorageType::Double);

    let now = Instant::now();
    fill_n(&mut hist, &ax1, &ax2, &ax1entries, &ax2entries, &weights);
    let elapsed = now.elapsed();
    println!("SparseHist (fill 10M): {:.2?}", elapsed);

    // HashMapHist benchmark
    let axes = vec![
        Box::new(ax1.clone()) as Box<dyn Axis>,
        Box::new(ax2.clone()) as Box<dyn Axis>,
    ];
    let mut hist = HashMapHist::new(axes, StorageType::Double);

    let now = Instant::now();
    fill_n(&mut hist, &ax1, &ax2, &ax1entries, &ax2entries, &weights);
    let elapsed = now.elapsed();
    println!("HashMapHist (fill 10M): {:.2?}", elapsed);
}
