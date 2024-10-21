use crate::axis::{Axis, AxisError};
use crate::bin::Bin;
use anyhow::Result;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub bins: Vec<Bin>,
}

impl Variable {
    pub fn new(name: String, edges: Vec<f64>) -> Self {
        // TODO: add checks
        let bins = edges
            .windows(2)
            .map(|w| Bin {
                low: w[0],
                high: w[1],
            })
            .collect();
        Self { name, bins }
    }
}

impl Axis for Variable {
    fn name(&self) -> &str {
        &self.name
    }

    fn bins(&self) -> &Vec<Bin> {
        &self.bins
    }

    fn num_bins(&self) -> usize {
        self.bins.len()
    }

    fn lower_bound(&self) -> f64 {
        self.bins[0].low
    }

    fn upper_bound(&self) -> f64 {
        self.bins[self.bins.len() - 1].high
    }

    fn index(&self, value: f64) -> Result<usize> {
        // find index with binary search
        // (this should be eytzinger layout for better cache performance)
        let mut low = 0;
        let mut high = self.bins.len() - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let bin = &self.bins[mid];
            if value >= bin.low && value < bin.high {
                return Ok(mid);
            } else if value < bin.low {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        Err(AxisError::FailedToFindBinIndex.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable() {
        let edges = vec![0.0, 1.0, 2.0, 3.0];
        let var = Variable::new("test".to_string(), edges);

        assert_eq!(var.name(), "test");
        assert_eq!(var.num_bins(), 3);
        assert_eq!(var.lower_bound(), 0.0);
        assert_eq!(var.upper_bound(), 3.0);

        assert_eq!(var.index(0.0).unwrap(), 0);
        assert_eq!(var.index(0.5).unwrap(), 0);
        assert_eq!(var.index(1.0).unwrap(), 1);
        assert_eq!(var.index(1.5).unwrap(), 1);
        assert_eq!(var.index(2.0).unwrap(), 2);
        assert_eq!(var.index(2.5).unwrap(), 2);
        assert!(var.index(5.0).is_err());
    }
}
