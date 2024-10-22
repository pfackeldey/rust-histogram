use crate::axis::{Axis, AxisError};
use crate::bin::SingleValue;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Integer {
    pub bins: Vec<SingleValue<i64>>,
}

impl Integer {
    // bin layout: [under-/overflow, bins]
    // under-/overflow bin is always at index 0
    // bins are at indices 1..=num
    // This layout allows for growing the number of bins
    pub fn new(bins: Vec<i64>) -> Result<Self> {
        if bins.is_empty() {
            return Err(AxisError::InvalidNumberOfBins.into());
        }

        let num = bins.len();

        let mut single_bins = Vec::with_capacity(num + 1);
        // under-/overflow bin
        single_bins.push(SingleValue::new(0));
        for bin in bins {
            single_bins.push(SingleValue::new(bin));
        }

        Ok(Self { bins: single_bins })
    }

    pub fn index(&self, value: i64) -> usize {
        match self.bins[1..].binary_search_by(|bin| bin.value.cmp(&value)) {
            Ok(index) => index + 1,
            Err(_) => self.overflow(),
        }
    }
}

impl Axis for Integer {
    fn num_bins(&self, flow: bool) -> usize {
        let num = self.bins.len() - 1;
        if flow {
            // include overflow bin
            // there is no underflow bin for integer axis
            // as it is `type BinType = SingleValue<...>`
            return num + 1;
        }
        num
    }

    fn underflow(&self) -> usize {
        0
    }

    fn overflow(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_axis() {
        // ungrowable axis
        let axis = Integer::new(vec![0, 1, 4, 8, 120]).unwrap();
        assert_eq!(axis.num_bins(false), 5);
        assert_eq!(axis.num_bins(true), 6);
        assert_eq!(axis.index(0), 1);
        assert_eq!(axis.index(1), 2);
        assert_eq!(axis.index(4), 3);
        assert_eq!(axis.index(8), 4);
        assert_eq!(axis.index(120), 5);
        // overflow
        assert_eq!(axis.index(123), 0);
    }
}
