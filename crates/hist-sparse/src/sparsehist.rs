use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;
use hist_storages::{Storage, StorageType};

// Holds the data as a Vec of filled bins
#[derive(Debug)]
pub struct SparseHist<'a, A: Axis> {
    pub axes: Vec<&'a A>,
    pub data: Vec<Storage>,
    pub data_indices: Vec<usize>,
    pub storage: StorageType,
}

impl<'a, A: Axis> SparseHist<'a, A> {
    pub fn new(axes: Vec<&'a A>, storage: StorageType) -> Self {
        Self {
            axes,
            data: Vec::new(),         // keeps track of the values of filled bins
            data_indices: Vec::new(), // keeps track of the indices of filled bins
            storage,
        }
    }
}

impl<'a, A: Axis> Histogram<A> for SparseHist<'a, A> {
    fn get_axes(&self) -> &Vec<&A> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> Storage {
        self.data_indices.iter().position(|&x| x == idx).map_or(
            match self.storage {
                StorageType::Double => Storage::Double(0.0),
                StorageType::Int => Storage::Int(0),
                StorageType::Weight => Storage::Weight((0.0, 0.0)),
            },
            |pos| self.data[pos].clone(),
        )
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
        // otherwise: push the bin to the data and data_indices vecs
        if let Some(idx) = self.data_indices.iter().position(|&x| x == bin_idx) {
            match &mut self.data[idx] {
                Storage::Double(val) => *val += weight,
                Storage::Int(val) => *val += weight as i64,
                Storage::Weight(val) => {
                    val.0 += weight;
                    val.1 += weight * weight;
                }
            }
        } else {
            self.data_indices.push(bin_idx);
            match self.storage {
                StorageType::Double => self.data.push(Storage::Double(weight)),
                StorageType::Int => self.data.push(Storage::Int(weight as i64)),
                StorageType::Weight => self.data.push(Storage::Weight((weight, weight * weight))),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sparsehist() {
        use hist::hist::Histogram;
        use hist_axes::uniform::Uniform;
        use hist_storages::{Storage, StorageType};

        let axis1 = Uniform::new(10, 0.0, 10.0).unwrap();
        let axis2 = Uniform::new(10, 0.0, 10.0).unwrap();

        let mut hist = super::SparseHist::new(vec![&axis1, &axis2], StorageType::Double);
        assert_eq!(hist.data.len(), 0);
        assert_eq!(hist.data_indices.len(), 0);

        let values = vec![0.5, 0.5];
        hist.fill(values, 1.0).unwrap();
        assert_eq!(hist.data.len(), 1);
        assert_eq!(hist.data_indices.len(), 1);

        assert_eq!(hist.get_bin(hist.data_indices[0]), Storage::Double(1.0));
    }
}
