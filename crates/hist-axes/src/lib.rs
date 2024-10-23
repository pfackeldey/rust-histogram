use anyhow::Result;
use axis::AxisError;
use category::Category;
use integer::Integer;
use uniform::Uniform;
use variable::Variable;

pub mod axis;
pub mod bin;
pub mod category;
pub mod integer;
pub mod uniform;
pub mod variable;

pub enum Axes {
    Uniform(Uniform),
    Variable(Variable),
    Category(Category),
    Integer(Integer),
}

pub trait AxisIndex<T> {
    fn index(&self, value: T) -> Result<usize>;
}

impl AxisIndex<f32> for Axes {
    fn index(&self, value: f32) -> Result<usize> {
        match self {
            Axes::Uniform(axis) => Ok(axis.index(value)),
            Axes::Variable(axis) => Ok(axis.index(value)),
            _ => Err(AxisError::InvalidValueType.into()),
        }
    }
}

impl AxisIndex<i32> for Axes {
    fn index(&self, value: i32) -> Result<usize> {
        match self {
            Axes::Integer(axis) => Ok(axis.index(value)),
            _ => Err(AxisError::InvalidValueType.into()),
        }
    }
}

impl AxisIndex<String> for Axes {
    fn index(&self, value: String) -> Result<usize> {
        match self {
            Axes::Category(axis) => Ok(axis.index(value)),
            _ => Err(AxisError::InvalidValueType.into()),
        }
    }
}
