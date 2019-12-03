use ndarray_unit::*;

extern crate ndarray;

use ndarray::{Array, ArrayBase};

fn main() {
    println!("{}", &get_meter() / &get_second());

    let arr1: ArrayBase<_, _> = Array::linspace(30.0, 40.0, 11);
    let okay1 = ArrayUnit::from_array_base(arr1, get_joule());

    let arr2 = Array::linspace(10.0, 60.0, 11);
    let okay2 = ArrayUnit::from_array_base(arr2, get_second());

    let arr3 = ndarray::array![
        [1.0, 0.0, 2.0, 6.0],
        [1.0, 2.0, 3.0, 5.0],
        [1.0, 2.0, 3.0, 6.0]
    ];
    let okay3 = ArrayUnit::from_array_base(arr3, get_meter());

    println!("{}\n{}\n{}", &okay1, &okay2, &okay3);
}
