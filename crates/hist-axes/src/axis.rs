use anyhow::Result;
use std::fmt::Debug;
use thiserror::Error;

use crate::bin::Bin;

pub trait Axis: Debug {
    fn name(&self) -> &str;
    fn bins(&self) -> &Vec<Bin>;
    fn num_bins(&self) -> usize;
    fn lower_bound(&self) -> f64;
    fn upper_bound(&self) -> f64;
    fn index(&self, value: f64) -> Result<usize>;
}

#[derive(Error, Debug)]
pub enum AxisError {
    #[error("number of bins should be positive and non-zero and must be convertible to the coordinate type")]
    InvalidNumberOfBins,
    #[error("axis step size should be non-zero and positive")]
    InvalidStepSize,
    #[error("failed to sort bins. The list of axis bin edges must be sortable.")]
    FailedToSortBins,
    #[error("failed to find bin index. The value is outside the axis range.")]
    FailedToFindBinIndex,
}
