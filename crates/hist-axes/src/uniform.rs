use std::fmt::{Debug, Formatter};

use crate::axis::{Axis, AxisError};
use crate::bin::Interval;
use anyhow::Result;

#[derive(Clone)]
pub struct Uniform {
    pub bins: Vec<Interval<f32>>,
    pub low: f32,
    pub high: f32,
    pub step: f32,
    pub num: usize,
}

impl Uniform {
    pub fn new(num: usize, start: f32, stop: f32) -> Result<Self> {
        if num == 0 {
            return Err(AxisError::InvalidNumberOfBins.into());
        }
        let step = (stop - start) / num as f32;
        if step <= 0.0 {
            return Err(AxisError::InvalidStepSize.into());
        }
        let mut bins = Vec::with_capacity(num);
        for i in 0..num {
            let lo = start + i as f32 * step;
            let hi = lo + step;
            bins.push(Interval::new(lo, hi));
        }
        Ok(Self {
            bins,
            low: start,
            high: stop,
            step,
            num,
        })
    }

    pub fn index(&self, value: f32) -> usize {
        // bin layout: [bins, underflow, overflow]
        if value < self.low {
            self.underflow()
        } else if value > self.high {
            self.overflow()
        } else {
            ((value - self.low) / self.step).floor() as usize
        }
    }
}

impl Axis for Uniform {
    fn num_bins(&self, flow: bool) -> usize {
        if flow {
            // include underflow and overflow bins
            return self.num + 2;
        }
        self.num
    }
}

impl Debug for Uniform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Uniform({:#?}..{:#?}, #{:#?} bins)",
            self.low,
            self.high,
            self.num_bins(false),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_axis() {
        let axis = Uniform::new(10, 0.0, 1.0).unwrap();
        assert_eq!(axis.num_bins(false), 10);
        assert_eq!(axis.num_bins(true), 12);
        assert_eq!(axis.index(0.0), 0);
        assert_eq!(axis.index(0.1), 1);
        assert_eq!(axis.index(0.9), 9);
    }
}
