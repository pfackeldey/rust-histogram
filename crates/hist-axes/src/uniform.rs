use crate::axis::{Axis, AxisError};
use crate::bin::Interval;
use anyhow::Result;
use num_traits::{Float, Num, NumCast, NumOps};

#[derive(Debug)]
pub struct Uniform<V = f64> {
    pub bins: Vec<Interval<V>>,
    pub low: V,
    pub high: V,
    pub step: V,
    pub num: usize,
}

impl<V> Uniform<V>
where
    V: PartialOrd + Num + NumCast + NumOps + Copy,
{
    pub fn new(num: usize, start: V, stop: V) -> Result<Self>
    where
        V: Float,
    {
        let step = (stop - start) / V::from(num).ok_or(AxisError::InvalidNumberOfBins)?;
        if step <= V::from(0.0).ok_or(AxisError::InvalidStepSize)? {
            return Err(AxisError::InvalidStepSize.into());
        }
        let mut bins = Vec::with_capacity(num);
        for i in 0..num {
            let lo = start + V::from(i).ok_or(AxisError::InvalidStepSize).unwrap() * step;
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
}

impl<V> Axis for Uniform<V>
where
    V: PartialOrd + Num + NumCast + NumOps + Copy + Clone,
{
    type ValueType = V;
    type BinType = Interval<V>;

    fn bins(&self) -> &Vec<Self::BinType> {
        &self.bins
    }

    fn num_bins(&self, flow: bool) -> usize {
        if flow {
            // include underflow and overflow bins
            return self.num + 2;
        }
        self.num
    }

    fn index(&self, value: Self::ValueType) -> usize {
        // bin layout: [bins, underflow, overflow]
        match value {
            v if v < self.low => self.underflow(),
            v if v > self.high => self.overflow(),
            _ => {
                let index = (value - self.low) / self.step;
                NumCast::from(index).unwrap()
            }
        }
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
