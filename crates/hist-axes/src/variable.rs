use crate::axis::{Axis, AxisError};
use crate::bin::Interval;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Variable {
    pub bins: Vec<Interval<f64>>,
}

impl Variable {
    pub fn new(edges: Vec<f64>) -> Result<Self> {
        if edges.len() < 2 {
            return Err(AxisError::InvalidNumberOfBinEdges.into());
        }
        if !edges.windows(2).all(|w| w[0] < w[1]) {
            return Err(AxisError::FailedToSortBins.into());
        }
        let bins = edges
            .windows(2)
            .map(|w| Interval::new(w[0], w[1]))
            .collect();
        Ok(Self { bins })
    }

    pub fn index(&self, value: f64) -> usize {
        // find index with binary search
        // (this should be eytzinger layout for better cache performance)
        // bin layout: [bins, underflow, overflow]
        match value {
            v if v < self.bins[0].low => self.underflow(),
            v if v > self.bins[self.bins.len() - 1].high => self.overflow(),
            _ => self
                .bins
                .binary_search_by(|bin| {
                    if bin.low <= value && value <= bin.high {
                        std::cmp::Ordering::Equal
                    } else if bin.low > value {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                })
                .unwrap(),
        }
    }
}

impl Axis for Variable {
    fn num_bins(&self, flow: bool) -> usize {
        if flow {
            // include underflow and overflow bins
            return self.bins.len() + 2;
        }
        self.bins.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_axis() {
        let edges: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0];
        let var = Variable::new(edges).unwrap();

        assert_eq!(var.num_bins(false), 3);
        assert_eq!(var.num_bins(true), 5);

        assert_eq!(var.index(0.0), 0);
        assert_eq!(var.index(0.5), 0);
        assert_eq!(var.index(1.0), 1);
        assert_eq!(var.index(1.5), 1);
        assert_eq!(var.index(2.0), 2);
        assert_eq!(var.index(2.5), 2);
    }
}
