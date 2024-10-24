use std::fmt::Debug;
use thiserror::Error;

pub trait Axis: Debug {
    // bin layout: [bins, underflow, overflow]
    fn num_bins(&self, flow: bool) -> usize;
    fn underflow(&self) -> usize {
        self.num_bins(true) - 2
    }
    fn overflow(&self) -> usize {
        self.num_bins(true) - 1
    }
}

#[derive(Error, Debug)]
pub enum AxisError {
    #[error("number of bins should be positive and non-zero and must be convertible to the coordinate type")]
    InvalidNumberOfBins,
    #[error("axis step size should be non-zero and positive")]
    InvalidStepSize,
    #[error("need to have at least two bin edges.")]
    InvalidNumberOfBinEdges,
    #[error("failed to sort bins. The list of axis bin edges must be sortable.")]
    FailedToSortBins,
    #[error("failed to find bin index. The value is outside the axis range.")]
    FailedToFindBinIndex,
    #[error("invalid value type for axis, can't be used for indexing.")]
    InvalidValueType,
}
