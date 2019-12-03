use ndarray_unit::*;

extern crate ndarray;

use ndarray::{Array, ArrayBase};

fn main() {
    println!("meter / second = {}", &get_meter() / &get_second());

    let arr1: ArrayBase<_, _> = Array::linspace(30.0, 40.0, 11);
    let arr_u1 = ArrayUnit::new(arr1, get_joule());

    let arr2 = Array::linspace(10.0, 60.0, 11);
    let arr_u2 = ArrayUnit::new(arr2, get_second());

    let arr3 = ndarray::array![
        [1.0, 0.0, 2.0, 6.0],
        [1.0, 2.0, 3.0, 5.0],
        [1.0, 2.0, 3.0, 6.0]
    ];
    let arr_u3 = ArrayUnit::new(arr3, get_meter());

    println!("arr_u3 = {}", arr_u3);
    println!("==========================================================");
    println!("{}\n*{}\n={}", &arr_u1, &arr_u2, &arr_u1 * &arr_u2);
    println!("==========================================================");
    println!("{}\n/{}\n={}", &arr_u1, &arr_u2, &arr_u1 / &arr_u2);
    println!("==========================================================");
    println!("{}\n+{}\n={}", &arr_u1, &arr_u1, &arr_u1 + &arr_u1);
    println!("==========================================================");
    println!("{}\n-{}\n={}", &arr_u2, &arr_u2, &arr_u2 - &arr_u2);
    println!("==========================================================");
}
