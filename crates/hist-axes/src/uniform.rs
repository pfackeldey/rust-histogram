use crate::axis::{Axis, AxisError};
use crate::bin::Bin;
use anyhow::Result;

#[derive(Debug)]
pub struct Uniform {
    pub name: String,
    pub bins: Vec<Bin>,
    pub low: f64,
    pub high: f64,
    pub step: f64,
    pub num: usize,
}

impl Uniform {
    pub fn new(name: String, start: f64, stop: f64, num: usize) -> Self {
        // TODO: add checks
        let step = (stop - start) / num as f64;
        let mut bins = Vec::with_capacity(num);
        for i in 0..num {
            let lo = start + i as f64 * step;
            let hi = lo + step;
            bins.push(Bin { low: lo, high: hi });
        }
        Self {
            name,
            bins,
            low: start,
            high: stop,
            step,
            num,
        }
    }
}

impl Axis for Uniform {
    fn name(&self) -> &str {
        &self.name
    }

    fn bins(&self) -> &Vec<Bin> {
        &self.bins
    }

    fn num_bins(&self) -> usize {
        self.num
    }

    fn lower_bound(&self) -> f64 {
        self.low
    }

    fn upper_bound(&self) -> f64 {
        self.high
    }

    fn index(&self, value: f64) -> Result<usize> {
        if value < self.low || value >= self.high {
            return Err(AxisError::FailedToFindBinIndex.into());
        }
        let idx = ((value - self.low) / self.step).floor() as usize;
        Ok(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_axis() {
        let axis = Uniform::new("test".to_string(), 0.0, 1.0, 10);
        assert_eq!(axis.num_bins(), 10);
        assert_eq!(axis.lower_bound(), 0.0);
        assert_eq!(axis.upper_bound(), 1.0);
        assert_eq!(axis.index(0.0).unwrap(), 0);
        assert_eq!(axis.index(0.1).unwrap(), 1);
        assert_eq!(axis.index(0.9).unwrap(), 9);
        assert!(axis.index(1.0).is_err());
    }
}
