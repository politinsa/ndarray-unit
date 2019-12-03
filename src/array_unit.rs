use std::fmt::{Display, Formatter, Result};
use std::ops::{Div, Mul};

extern crate ndarray;
use ndarray::{ArrayBase, Data, Dimension};

use crate::unit::Unit;

pub struct ArrayUnit<T, D>
where
    T: Data,
    D: Dimension,
{
    unit: Unit,
    array: ArrayBase<T, D>,
}

impl<T, D> ArrayUnit<T, D>
where
    T: Data,
    D: Dimension,
{
    pub fn from_array_base(arr: ArrayBase<T, D>, u: Unit) -> ArrayUnit<T, D> {
        ArrayUnit {
            unit: u,
            array: arr,
        }
    }
}

impl<T, D> Mul for &ArrayUnit<T, D>
where
    T: Data,
    D: Dimension,
    for<'a> &'a ArrayBase<T, D>: Mul<Output = ArrayBase<T, D>>,
{
    type Output = ArrayUnit<T, D>;

    fn mul(self, other: &ArrayUnit<T, D>) -> ArrayUnit<T, D> {
        ArrayUnit {
            unit: &self.unit * &other.unit,
            array: &self.array * &other.array,
        }
    }
}

impl<T, D> Div for &ArrayUnit<T, D>
where
    T: Data,
    D: Dimension,
    for<'a> &'a ArrayBase<T, D>: Div<Output = ArrayBase<T, D>>,
{
    type Output = ArrayUnit<T, D>;

    fn div(self, other: &ArrayUnit<T, D>) -> ArrayUnit<T, D> {
        ArrayUnit {
            unit: &self.unit / &other.unit,
            array: &self.array / &other.array,
        }
    }
}

impl<A: Display, T, D> Display for ArrayUnit<T, D>
where
    T: Data<Elem = A>,
    D: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", &self.array, &self.unit)
    }
}