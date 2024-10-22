use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;
use hist_storages::{Storage, StorageType};
use std::fmt::{Debug, Formatter};

// Holds the data as a flat vector
pub struct VecHist {
    pub axes: Vec<Box<dyn Axis>>,
    pub data: Vec<Storage>,
    pub storage: StorageType,
}

impl VecHist {
    pub fn new(axes: Vec<Box<dyn Axis>>, storage: StorageType) -> Self {
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

impl Histogram for VecHist {
    fn get_axes(&self) -> &Vec<Box<dyn Axis>> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> Storage {
        self.data[idx].clone()
    }

    fn fill(&mut self, indices: &Vec<usize>, weight: f64) -> Result<()> {
        let axes = self.get_axes();

        if indices.len() != axes.len() {
            return Err(HistError::AxesValuesMismatch {
                nvalues: indices.len(),
                naxes: axes.len(),
            }
            .into());
        }

        // Find the index of the bin
        // and fill the bin with the weight
        let bin_idx = self.stride_index(indices)?;
        match self.storage {
            StorageType::Double => self.data[bin_idx] += Storage::Double(weight),
            StorageType::Int => self.data[bin_idx] += Storage::Int(weight as i64),
            StorageType::Weight => self.data[bin_idx] += Storage::Weight((weight, weight * weight)),
        }

        Ok(())
    }
}

impl Debug for VecHist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VecHist(axes={:?}, storage={:?})",
            self.axes, self.storage
        )
        // let primitive_bins = self
        //     .data
        //     .iter()
        //     .map(|s| match s {
        //         Storage::Double(val) => *val,
        //         Storage::Int(val) => *val as f64,
        //         Storage::Weight((val, _)) => *val,
        //     })
        //     .collect::<Vec<f64>>();
        // let sum = primitive_bins.iter().sum::<f64>();
        // write!(f, "\nBins (sumw={:?}): {:?}", sum, primitive_bins)
    }
}

#[cfg(test)]
mod tests {
    use hist_axes::axis::Axis;

    #[test]
    fn test_vechist() {
        use hist::hist::Histogram;
        use hist_axes::category::Category;
        use hist_axes::uniform::Uniform;
        use hist_storages::{Storage, StorageType};

        let uniform = Uniform::new(10, 0.0, 10.0).unwrap();
        let cat = Category::new(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ])
        .unwrap();

        let axes = vec![
            Box::new(uniform.clone()) as Box<dyn Axis>,
            Box::new(cat.clone()) as Box<dyn Axis>,
        ];
        let mut hist = super::VecHist::new(axes, StorageType::Double);
        assert_eq!(hist.get_axes().len(), 2);
        assert_eq!(hist.num_bins(false), 50);
        assert_eq!(hist.num_bins(true), 72);

        // Fill the histogram
        let where2fill = vec![uniform.index(0.0), cat.index("A".to_string())];
        hist.fill(&where2fill, 1.0).unwrap();

        assert_eq!(hist.num_bins(false), 50);
        assert_eq!(hist.num_bins(true), 72);
        assert_eq!(hist.get_bin(0), Storage::Double(0.0));

        // Check the bin
        assert_eq!(
            hist.get_bin(hist.stride_index(&where2fill).unwrap()),
            Storage::Double(1.0)
        );
    }
}
