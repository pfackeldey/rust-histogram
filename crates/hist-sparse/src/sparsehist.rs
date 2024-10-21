use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;

// Holds the data as a hashmap
#[derive(Debug)]
pub struct SparseHist {
    pub axes: Vec<Box<dyn Axis>>,
    pub data: Vec<f64>,
    pub data_indices: Vec<usize>,
}

impl SparseHist {
    pub fn new(axes: Vec<Box<dyn Axis>>) -> Self {
        Self {
            axes,
            data: Vec::new(),         // keeps track of the values of filled bins
            data_indices: Vec::new(), // keeps track of the indices of filled bins
        }
    }
}

impl Histogram for SparseHist {
    fn get_axes(&self) -> &Vec<Box<dyn Axis>> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> f64 {
        self.data_indices
            .iter()
            .find(|&&x| x == idx)
            .map_or(0.0, |&x| self.data[x])
    }

    fn fill(&mut self, values: Vec<f64>, weight: f64) -> Result<()> {
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
        // in the data vector and the data_indices vector
        match self.data_indices.iter().position(|&x| x == bin_idx) {
            Some(idx) => {
                self.data[idx] += weight;
            }
            None => {
                self.data_indices.push(bin_idx);
                self.data.push(weight);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hashmaphist() {
        use hist::hist::Histogram;
        use hist_axes::axis::Axis;
        use hist_axes::uniform::Uniform;

        let uniform1 = Uniform::new("axis1".to_string(), 0.0, 10.0, 10);
        let uniform2 = Uniform::new("axis2".to_string(), 0.0, 10.0, 10);

        let axis1 = Box::new(uniform1) as Box<dyn Axis>;
        let axis2 = Box::new(uniform2) as Box<dyn Axis>;
        let axes = vec![axis1, axis2];

        let mut hist = super::SparseHist::new(axes);
        assert_eq!(hist.data.len(), 0);
        assert_eq!(hist.data_indices.len(), 0);

        let values = vec![0.5, 0.5];
        hist.fill(values, 1.0).unwrap();
        assert_eq!(hist.data.len(), 1);
        assert_eq!(hist.data_indices.len(), 1);

        assert_eq!(hist.get_bin(hist.data_indices[0]), 1.0);
    }
}
