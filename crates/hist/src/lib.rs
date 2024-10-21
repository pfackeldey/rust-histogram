// General histogram interface:
//
// We want to add multiple different types of histograms,
// but they all need to be able to index and fill.
//
// Envisioned histogram types:
// - VecHist: holds the data as a flat vector
// - HashMapHist: holds only the filled data as a HashMap
// - SparseHist: holds only the filled data and indices as a Vec each
// - NDArrayHist: holds the data as an ndarray
// - ZarrHist: holds the data (compressed) as a zarr array
pub mod hist {
    use anyhow::Result;
    use hist_axes::axis::Axis;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum HistError {
        #[error("mismatch in number of values ({nvalues}) and axes ({naxes})")]
        AxesValuesMismatch { nvalues: usize, naxes: usize },
    }

    pub trait Histogram {
        fn get_axes(&self) -> &Vec<Box<dyn Axis>>;

        fn num_bins(&self) -> usize {
            // Assuming that the trait implementer will have a method or a way to provide axes
            let axes = self.get_axes();
            axes.iter().map(|axis| axis.num_bins()).product()
        }

        fn index(&self, values: Vec<f64>) -> Result<Vec<usize>> {
            let axes = self.get_axes();
            if values.len() != axes.len() {
                return Err(HistError::AxesValuesMismatch {
                    nvalues: values.len(),
                    naxes: axes.len(),
                }
                .into());
            }

            let mut indices = Vec::new();
            for (axis, value) in axes.iter().zip(values.iter()) {
                let idx = axis.index(*value)?;
                indices.push(idx);
            }
            Ok(indices)
        }

        fn find_bin_index(&self, values: Vec<f64>) -> Result<usize> {
            let axes = self.get_axes();
            if values.len() != axes.len() {
                return Err(HistError::AxesValuesMismatch {
                    nvalues: values.len(),
                    naxes: axes.len(),
                }
                .into());
            }

            let mut flat_index = 0;
            for (axis, value) in axes.iter().zip(values.iter()) {
                let idx = axis.index(*value)?;
                flat_index = flat_index * axis.num_bins() + idx;
            }
            Ok(flat_index)
        }

        fn get_bin(&self, idx: usize) -> f64;

        fn fill(&mut self, values: Vec<f64>, weight: f64) -> Result<()>;
    }
}
