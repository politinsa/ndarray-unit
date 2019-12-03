![](https://github.com/politinsa/ndarray-unit/workflows/Build/badge.svg)
![](https://github.com/politinsa/ndarray-unit/workflows/Tests/badge.svg)
[![crates.io](https://meritbadge.herokuapp.com/ndarray-unit)](https://crates.io/crates/ndarray-unit)

# ndarray-unit

* Homepage on [crates.io](https://crates.io/crates/ndarray-unit)
* **Doc** on [doc.rs](https://docs.rs/ndarray-unit/)  

This crate provides a struct representing a [ndarray::ArrayBase](https://docs.rs/ndarray/) together with a [Unit](struct.Unit.html).
It allows to do computations taking into account the unit of your n-dimensional array.

## Examples

```rust
use ndarray_unit::*;

extern crate ndarray;
use ndarray::Array;

fn main() {
    println!("meter / second = {}", &get_meter() / &get_second());

    let arr1 = Array::linspace(30.0, 40.0, 11);
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
```
**Output**
```rust
// meter / second = m·s⁻¹
// arr_u3 = [[1, 0, 2, 6],
//  [1, 2, 3, 5],
//  [1, 2, 3, 6]] m
// ==========================================================
// [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
// *[10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
// =[300, 465, 640, 825, 1020, 1225, 1440, 1665, 1900, 2145, 2400] m²·kg·s⁻¹
// ==========================================================
// [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
// /[10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
// =[3, 2.0666, 1.6, 1.32, 1.1333, 1, 0.9, 0.8222, 0.76, 0.7090, 0.6666] m²·kg·s⁻³
// ==========================================================
// [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
// +[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
// =[60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80] m²·kg·s⁻²
// ==========================================================
// [10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
// -[10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
// =[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] s
// ==========================================================
```

## Panics
The program will panic when you try to add or substract two [ArrayUnit](struct.ArrayUnit.html)s with different [Unit](struct.Unit.html)s.
```rust
extern crate ndarray;
use ndarray::Array;
use ndarray_unit::*;

let arr1 = Array::linspace(30.0, 40.0, 11);
let arr_u1 = ArrayUnit::new(arr1, get_joule());

let arr2 = Array::linspace(10.0, 60.0, 11);
let arr_u2 = ArrayUnit::new(arr2, get_second());

// let result = &arr_u1 + &arr_u2; // ==> panicking
```

## Development
* Doc of the master branch on [github.io](https://politinsa.github.io/ndarray-unit/)
