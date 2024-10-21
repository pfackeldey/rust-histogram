use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;
use hist_storages::{Storage, StorageType};
use std::collections::HashMap;

// Holds the data as a hashmap
#[derive(Debug, Clone)]
pub struct HashMapHist<'a, A: Axis> {
    pub axes: Vec<&'a A>,
    pub data: HashMap<usize, Storage>,
    pub storage: StorageType,
}

impl<'a, A: Axis> HashMapHist<'a, A> {
    pub fn new(axes: Vec<&'a A>, storage: StorageType) -> Self {
        Self {
            axes,
            data: HashMap::new(),
            storage,
        }
    }
}

impl<'a, A: Axis> Histogram<A> for HashMapHist<'a, A> {
    fn get_axes(&self) -> &Vec<&A> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> Storage {
        self.data
            .get(&idx)
            .cloned()
            .unwrap_or_else(|| match self.storage {
                StorageType::Double => Storage::Double(0.0),
                StorageType::Int => Storage::Int(0),
                StorageType::Weight => Storage::Weight((0.0, 0.0)),
            })
    }

    fn fill(&mut self, values: Vec<A::ValueType>, weight: f64) -> Result<()> {
        let axes = self.get_axes();

        if values.len() != axes.len() {
            return Err(HistError::AxesValuesMismatch {
                nvalues: values.len(),
                naxes: axes.len(),
            }
            .into());
        }

        // Find the index of the bin for each axis
        let bin_idx = self.find_bin_index(values)?;

        // Increment the bin by the weight
        // if the bin exists: increment the bin inplace
        // otherwise: insert the bin
        match self.data.get_mut(&bin_idx) {
            Some(val) => {
                match self.storage {
                    StorageType::Double => *val += Storage::Double(weight),
                    StorageType::Int => *val += Storage::Int(weight as i64),
                    StorageType::Weight => *val += Storage::Weight((weight, weight * weight)),
                }
                Ok(())
            }
            None => {
                let init_val = match self.storage {
                    StorageType::Double => Storage::Double(weight),
                    StorageType::Int => Storage::Int(weight as i64),
                    StorageType::Weight => Storage::Weight((weight, weight * weight)),
                };
                self.data.insert(bin_idx, init_val);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hashmaphist() {
        use hist::hist::Histogram;
        use hist_axes::uniform::Uniform;
        use hist_storages::{Storage, StorageType};

        let axis1 = Uniform::new(10, 0.0, 10.0).unwrap();
        let axis2 = Uniform::new(10, 0.0, 10.0).unwrap();

        let mut hist = super::HashMapHist::new(vec![&axis1, &axis2], StorageType::Double);
        assert_eq!(hist.data.len(), 0);

        let values = vec![0.5, 0.5];
        hist.fill(values, 1.0).unwrap();
        assert_eq!(hist.data.len(), 1);

        assert_eq!(hist.get_bin(0), Storage::Double(1.0));
        assert_eq!(hist.get_bin(1), Storage::Double(0.0));
    }
}
