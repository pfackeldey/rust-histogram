use crate::axis::{Axis,Fill};

#[derive(Debug)]
pub struct Histogram {
    pub axis: Axis,
}

impl Histogram {
    pub fn new(axis: Axis) -> Self {
        Histogram { axis }
    }
}

impl Fill for Histogram {
    fn weighted_fill(&mut self, value: f64, weight: f64) -> Result<(), String> {
        self.axis.weighted_fill(value, weight)?;
        Ok(())
    }
}
