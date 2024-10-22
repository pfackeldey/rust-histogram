use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::axis::{Axis, AxisError};
use crate::bin::SingleValue;
use anyhow::Result;

#[derive(Clone)]
pub struct Category {
    pub idx2bin: HashMap<usize, SingleValue<String>>,
    pub bin2idx: HashMap<String, usize>,
}

impl Category {
    // reserved under-/overflow bin name
    const OVERFLOW: &'static str = "__overflow__";
    // bin layout: [under-/overflow, bins]
    // under-/overflow bin is always at index 0
    // bins are at indices 1..=num
    // This layout allows for growing the number of bins
    pub fn new(bins: Vec<String>) -> Result<Self> {
        if bins.is_empty() {
            return Err(AxisError::InvalidNumberOfBins.into());
        }

        let num = bins.len();

        let mut idx2bin = HashMap::with_capacity(num + 1);
        let mut bin2idx = HashMap::with_capacity(num + 1);
        // under-/overflow bin
        idx2bin.insert(0, SingleValue::new(Self::OVERFLOW.to_string()));
        bin2idx.insert(Self::OVERFLOW.to_string(), 0);
        for (idx, bin) in bins.iter().enumerate() {
            // check for reserved bin name
            assert!(bin != &Self::OVERFLOW.to_string());
            idx2bin.insert(idx + 1, SingleValue::new(bin.clone()));
            bin2idx.insert(bin.clone(), idx + 1);
        }

        Ok(Self { idx2bin, bin2idx })
    }

    pub fn index(&self, value: String) -> usize {
        match self.bin2idx.get(&value.to_string()) {
            Some(&index) => index,
            None => self.overflow(),
        }
    }
}

impl Axis for Category {
    fn num_bins(&self, flow: bool) -> usize {
        let num = self.idx2bin.len() - 1;
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

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self
            .idx2bin
            .values()
            .map(|bin| bin.value.clone())
            .filter(|v| v != &Category::OVERFLOW.to_string())
            .collect();
        write!(
            f,
            "Category({:?}, #{:#?} bins)",
            names,
            self.num_bins(false),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_axis() {
        // ungrowable axis
        let axis = Category::new(vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ])
        .unwrap();
        assert_eq!(axis.num_bins(false), 3);
        assert_eq!(axis.num_bins(true), 4);
        assert_eq!(axis.index("foo".to_string()), 1);
        assert_eq!(axis.index("bar".to_string()), 2);
        assert_eq!(axis.index("baz".to_string()), 3);
        // overflow
        assert_eq!(axis.index(Category::OVERFLOW.to_string()), 0);
    }
}
