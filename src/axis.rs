#[derive(Debug)]
pub struct Bin {
    pub bounds: (f64, f64),
    pub value: f64,
}

impl Bin {
    pub fn new(lower: f64, upper: f64) -> Self {
        Bin { bounds: (lower, upper), value: 0.0 }
    }
}

pub trait Fill {
    fn weighted_fill(&mut self, value: f64, weight: f64) -> Result<(), String>;

    fn fill(&mut self, value: f64) -> Result<(), String> {
        let out = self.weighted_fill(value, 1.0)?;
        Ok(out)
    }
}

#[derive(Debug)]
pub struct Axis {
    pub name: String,
    pub bins: Vec<Bin>,
}

impl Axis {
    pub fn new(name: String, bins: Vec<Bin>) -> Self {
        for i in 0..bins.len() - 1 {
            assert!(
                bins[i].bounds.1 == bins[i + 1].bounds.0,
                "Bins must align their edges! Bin {} upper edge is not equal to Bin {} lower edge.",
                i,
                i + 1
            );
        }

        Axis { name, bins }
    }

    pub fn from_edges(name: String, edges: Vec<f64>) -> Self {
        let bins = edges
            .windows(2)
            .map(|w| Bin::new(w[0], w[1]))
            .collect();

        Axis::new(name, bins)
    }

    pub fn from_uniform(name: String, start: f64, stop: f64, num: usize) -> Self {
        let mut bins = Vec::with_capacity(num);

        let step = (stop - start) / num as f64;

        for i in 0..num {
            let lower = start + i as f64 * step;
            let upper = start + (i + 1) as f64 * step;
            bins.push(Bin::new(lower, upper));
        }

        Axis::new(name, bins)
    }
}

impl Fill for Axis {
    fn weighted_fill(&mut self, value: f64, weight: f64) -> Result<(), String> {
        // binary search for the bin index
        let mut left = 0;
        let mut right = self.bins.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let (lower, upper) = self.bins[mid].bounds;
            if value >= lower && value < upper {
                self.bins[mid].value += weight;
                return Ok(());
            } else if value < lower {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        Err(format!("Value {} is out of range for axis {}", value, self.name))
    }
}
