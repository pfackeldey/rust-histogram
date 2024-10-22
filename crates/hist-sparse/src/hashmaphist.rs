use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;
use hist_storages::{Storage, StorageType};
use std::collections::HashMap;

// Holds the data as a hashmap
pub struct HashMapHist {
    pub axes: Vec<Box<dyn Axis>>,
    pub data: HashMap<usize, Storage>,
    pub storage: StorageType,
}

impl HashMapHist {
    pub fn new(axes: Vec<Box<dyn Axis>>, storage: StorageType) -> Self {
        Self {
            axes,
            data: HashMap::new(),
            storage,
        }
    }
}

impl Histogram for HashMapHist {
    fn get_axes(&self) -> &Vec<Box<dyn Axis>> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> Storage {
        self.data.get(&idx).cloned().unwrap_or(match self.storage {
            StorageType::Double => Storage::Double(0.0),
            StorageType::Int => Storage::Int(0),
            StorageType::Weight => Storage::Weight((0.0, 0.0)),
        })
    }

    fn fill(&mut self, indices: Vec<usize>, weight: f64) -> Result<()> {
        let axes = self.get_axes();

        if indices.len() != axes.len() {
            return Err(HistError::AxesValuesMismatch {
                nvalues: indices.len(),
                naxes: axes.len(),
            }
            .into());
        }

        // Find the stride index of the bin for each axis
        let bin_idx = self.stride_index(indices)?;

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
        use hist_axes::axis::Axis;
        use hist_axes::uniform::Uniform;
        use hist_storages::{Storage, StorageType};

        let axis1 = Uniform::new(10, 0.0, 10.0).unwrap();
        let axis2 = Uniform::new(10, 0.0, 10.0).unwrap();

        let axes = vec![
            Box::new(axis1.clone()) as Box<dyn Axis>,
            Box::new(axis2.clone()) as Box<dyn Axis>,
        ];
        let mut hist = super::HashMapHist::new(axes, StorageType::Double);
        assert_eq!(hist.data.len(), 0);

        let where2fill = vec![axis1.index(0.5), axis2.index(0.5)];
        hist.fill(where2fill, 1.0).unwrap();
        assert_eq!(hist.data.len(), 1);

        assert_eq!(hist.get_bin(0), Storage::Double(1.0));
        assert_eq!(hist.get_bin(1), Storage::Double(0.0));
    }
}
