use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;
use hist_storages::{Storage, StorageType};

// Holds the data as a flat vector
#[derive(Debug, Clone)]
pub struct VecHist<'a, A: Axis> {
    pub axes: Vec<&'a A>,
    pub data: Vec<Storage>,
    pub storage: StorageType,
}

impl<'a, A: Axis> VecHist<'a, A> {
    pub fn new(axes: Vec<&'a A>, storage: StorageType) -> Self {
        let dims = axes.iter().map(|axis| axis.num_bins(true)).product();

        let init_val = match storage {
            StorageType::Double => Storage::Double(0.0),
            StorageType::Int => Storage::Int(0),
            StorageType::Weight => Storage::Weight((0.0, 0.0)),
        };

        let data = vec![init_val; dims];

        Self {
            axes,
            data,
            storage,
        }
    }
}

impl<'a, A: Axis> Histogram<A> for VecHist<'a, A> {
    fn get_axes(&self) -> &Vec<&A> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> Storage {
        self.data[idx].clone()
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

        // Find the index of the bin
        // and fill the bin with the weight
        let idx = self.find_bin_index(values)?;
        match self.storage {
            StorageType::Double => self.data[idx] += Storage::Double(weight),
            StorageType::Int => self.data[idx] += Storage::Int(weight as i64),
            StorageType::Weight => self.data[idx] += Storage::Weight((weight, weight * weight)),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vechist() {
        use hist::hist::Histogram;
        use hist_axes::uniform::Uniform;
        use hist_storages::{Storage, StorageType};

        let axis1 = Uniform::new(10, 0.0, 10.0).unwrap();
        let axis2 = Uniform::new(10, 0.0, 10.0).unwrap();

        let mut hist = super::VecHist::new(vec![&axis1, &axis2], StorageType::Double);
        assert_eq!(hist.get_axes().len(), 2);
        assert_eq!(hist.num_bins(false), 100);
        assert_eq!(hist.num_bins(true), 144);

        hist.fill(vec![0.0, 0.0], 1.0).unwrap();

        assert_eq!(hist.num_bins(false), 100);
        assert_eq!(hist.num_bins(true), 144);
        assert_eq!(hist.get_bin(0), Storage::Double(1.0));
    }
}
