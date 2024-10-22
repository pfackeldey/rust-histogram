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
    use hist_storages::Storage;
    use std::fmt::Debug;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum HistError {
        #[error("mismatch in number of values ({nvalues}) and axes ({naxes})")]
        AxesValuesMismatch { nvalues: usize, naxes: usize },
    }

    // General histogram interface
    pub trait Histogram {
        fn get_axes(&self) -> &Vec<Box<dyn Axis>>;

        fn num_bins(&self, flow: bool) -> usize {
            // Assuming that the trait implementer will have a method or a way to provide axes
            let axes = self.get_axes();
            axes.iter().map(|axis| axis.num_bins(flow)).product()
        }

        fn stride_index(&self, indices: &Vec<usize>) -> Result<usize> {
            let axes = self.get_axes();
            if indices.len() != axes.len() {
                return Err(HistError::AxesValuesMismatch {
                    nvalues: indices.len(),
                    naxes: axes.len(),
                }
                .into());
            }

            let mut strided_index = 0;
            for (axis, idx) in axes.iter().zip(indices.iter()) {
                let stride = axis.num_bins(true);
                strided_index = strided_index * stride + idx;
            }
            Ok(strided_index)
        }

        fn get_bin(&self, idx: usize) -> Storage;

        fn fill(&mut self, values: &Vec<usize>, weight: f64) -> Result<()>;

        fn fill_n(&mut self, values: &Vec<Vec<usize>>, weights: &Vec<f64>) -> Result<()> {
            for (values, weight) in values.iter().zip(weights.iter()) {
                self.fill(values, *weight)?;
            }
            Ok(())
        }
    }
}
