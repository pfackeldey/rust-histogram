use anyhow::Result;
use hist::hist::{HistError, Histogram};
use hist_axes::axis::Axis;

// Holds the data as a flat vector
#[derive(Debug)]
pub struct VecHist {
    pub axes: Vec<Box<dyn Axis>>,
    pub data: Vec<f64>,
}

impl VecHist {
    pub fn new(axes: Vec<Box<dyn Axis>>) -> Self {
        let dims = axes.iter().map(|axis| axis.num_bins()).product();
        let data = vec![0.0; dims];
        Self { axes, data }
    }
}

impl Histogram for VecHist {
    fn get_axes(&self) -> &Vec<Box<dyn Axis>> {
        &self.axes
    }

    fn get_bin(&self, idx: usize) -> f64 {
        self.data.get(idx).map_or(0.0, |&x| x)
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

        // Find the index of the bin
        let flat_index = self.find_bin_index(values)?;

        // Increment the bin by the weight
        // in the flat data vector
        self.data[flat_index] += weight;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vechist() {
        use hist::hist::Histogram;
        use hist_axes::axis::Axis;
        use hist_axes::uniform::Uniform;

        let uniform1 = Uniform::new("axis1".to_string(), 0.0, 10.0, 10);
        let uniform2 = Uniform::new("axis2".to_string(), 0.0, 10.0, 10);

        let axis1 = Box::new(uniform1) as Box<dyn Axis>;
        let axis2 = Box::new(uniform2) as Box<dyn Axis>;
        let axes = vec![axis1, axis2];

        let mut hist = super::VecHist::new(axes);
        assert_eq!(hist.get_axes().len(), 2);
        assert_eq!(hist.data.len(), 100);

        hist.fill(vec![0.0, 0.0], 1.0).unwrap();

        assert_eq!(hist.data.len(), 100);
        assert_eq!(hist.get_bin(0), 1.0);
    }
}
