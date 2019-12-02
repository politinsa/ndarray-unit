use ndarray_unit::*;

extern crate ndarray;

use ndarray::Array;

fn main() {
    println!("{}", &get_meter() / &get_second());
    println!("{}", Array::linspace(0.0,10.0,11));
}

