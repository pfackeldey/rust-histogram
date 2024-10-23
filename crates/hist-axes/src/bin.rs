use num_traits::{Num, NumCast, NumOps};

#[derive(Debug, Clone)]
pub struct Interval<V> {
    pub low: V,
    pub high: V,
}

impl<V> Interval<V>
where
    V: PartialOrd + Num + NumCast + NumOps + Copy,
{
    pub fn new(low: V, high: V) -> Self {
        assert!(high > low, "high must be greater than low");
        Self { low, high }
    }

    pub fn center(&self) -> V {
        (self.low + self.high) / V::from(2.0).unwrap()
    }

    pub fn width(&self) -> V {
        self.high - self.low
    }
}

#[derive(Debug, Clone)]
pub struct SingleValue<V> {
    pub value: V,
}

impl<V> SingleValue<V> {
    pub fn new(value: V) -> Self {
        Self { value }
    }
}
