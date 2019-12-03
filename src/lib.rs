//! This crate provides a struct representing a [ndarray::ArrayBase](https://docs.rs/ndarray/) together with a [Unit](struct.Unit.html).  
//! It allows to do computations taking into account the unit of your n-dimensional array.
//!
//! # Examples
//!
//! ```
//! use ndarray_unit::*;
//!
//! extern crate ndarray;
//! use ndarray::Array;
//!
//! fn main() {
//!     println!("meter / second = {}", &get_meter() / &get_second());
//!
//!     let arr1 = Array::linspace(30.0, 40.0, 11);
//!     let arr_u1 = ArrayUnit::new(arr1, get_joule());
//!
//!     let arr2 = Array::linspace(10.0, 60.0, 11);
//!     let arr_u2 = ArrayUnit::new(arr2, get_second());
//!
//!     let arr3 = ndarray::array![
//!         [1.0, 0.0, 2.0, 6.0],
//!         [1.0, 2.0, 3.0, 5.0],
//!         [1.0, 2.0, 3.0, 6.0]
//!     ];
//!     let arr_u3 = ArrayUnit::new(arr3, get_meter());
//!
//!     println!("arr_u3 = {}", arr_u3);
//!     println!("==========================================================");
//!     println!("{}\n*{}\n={}", &arr_u1, &arr_u2, &arr_u1 * &arr_u2);
//!     println!("==========================================================");
//!     println!("{}\n/{}\n={}", &arr_u1, &arr_u2, &arr_u1 / &arr_u2);
//!     println!("==========================================================");
//!     println!("{}\n+{}\n={}", &arr_u1, &arr_u1, &arr_u1 + &arr_u1);
//!     println!("==========================================================");
//!     println!("{}\n-{}\n={}", &arr_u2, &arr_u2, &arr_u2 - &arr_u2);
//!     println!("==========================================================");
//! }
//! ```
//! **Output**
//! ```
//! // meter / second = m·s⁻¹
//! // arr_u3 = [[1, 0, 2, 6],
//! //  [1, 2, 3, 5],
//! //  [1, 2, 3, 6]] m
//! // ==========================================================
//! // [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
//! // *[10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
//! // =[300, 465, 640, 825, 1020, 1225, 1440, 1665, 1900, 2145, 2400] m²·kg·s⁻¹
//! // ==========================================================
//! // [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
//! // /[10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
//! // =[3, 2.0666, 1.6, 1.32, 1.1333, 1, 0.9, 0.8222, 0.76, 0.7090, 0.6666] m²·kg·s⁻³
//! // ==========================================================
//! // [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
//! // +[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40] m²·kg·s⁻²
//! // =[60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80] m²·kg·s⁻²
//! // ==========================================================
//! // [10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
//! // -[10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60] s
//! // =[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] s
//! // ==========================================================
//! ```
//!
//! # Panics
//! The program will panic when you try to add or substract two [ArrayUnit](struct.ArrayUnit.html)s with different [Unit](struct.Unit.html)s.
//! ```
//! extern crate ndarray;
//! use ndarray::Array;
//! use ndarray_unit::*;
//!
//! let arr1 = Array::linspace(30.0, 40.0, 11);
//! let arr_u1 = ArrayUnit::new(arr1, get_joule());
//!
//! let arr2 = Array::linspace(10.0, 60.0, 11);
//! let arr_u2 = ArrayUnit::new(arr2, get_second());
//!
//! // let result = &arr_u1 + &arr_u2; // ==> panicking
//! ```

#![crate_name = "ndarray_unit"]

mod unit;
pub use unit::BaseUnit;
pub use unit::Unit;

mod array_unit;
pub use array_unit::ArrayUnit;

//////////////////////////
// Getters for base units
//////////////////////////

/// Utility method to get a Unit from a BaseUnit (BaseUnit::METER)
pub fn get_meter() -> Unit {
    Unit::from_vec(vec![(BaseUnit::METER, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::SECOND)
pub fn get_second() -> Unit {
    Unit::from_vec(vec![(BaseUnit::SECOND, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::CANDELA)
pub fn get_candela() -> Unit {
    Unit::from_vec(vec![(BaseUnit::CANDELA, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::MOLE)
pub fn get_mole() -> Unit {
    Unit::from_vec(vec![(BaseUnit::MOLE, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::KELVIN)
pub fn get_kelvin() -> Unit {
    Unit::from_vec(vec![(BaseUnit::KELVIN, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::AMPERE)
pub fn get_ampere() -> Unit {
    Unit::from_vec(vec![(BaseUnit::AMPERE, 1)])
}

/////////////////////////////
// Getters for other useful units
/////////////////////////////

/// Utility method to get a Unit from a BaseUnit (BaseUnit::RADIAN)
pub fn get_radian() -> Unit {
    Unit::from_vec(vec![(BaseUnit::RADIAN, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::STERADIAN)
pub fn get_steradian() -> Unit {
    Unit::from_vec(vec![(BaseUnit::STERADIAN, 1)])
}

/////////////////////////////
// Getters for economics indicators
/////////////////////////////
pub fn get_currency() -> Unit {
    Unit::from_vec(vec![(BaseUnit::CURRENCY, 1)])
}

pub fn get_birth() -> Unit {
    Unit::from_vec(vec![(BaseUnit::BIRTH, 1)])
}

pub fn get_death() -> Unit {
    Unit::from_vec(vec![(BaseUnit::DEATH, 1)])
}

pub fn get_inhabitant() -> Unit {
    Unit::from_vec(vec![(BaseUnit::INHABITANT, 1)])
}

/////////////////////////////
// Getters for composed units
/////////////////////////////

/// Utility method to get the Joule Unit (composed)
pub fn get_newton() -> Unit {
    Unit::from_vec(vec![
        (BaseUnit::KILOGRAM, 1),
        (BaseUnit::METER, 1),
        (BaseUnit::SECOND, -2),
    ])
}

/// Utility method to get the Joule Unit (composed)
pub fn get_joule() -> Unit {
    &get_newton() * &get_meter()
}

/// Utility method to get the Watt Unit (composed)
pub fn get_watt() -> Unit {
    Unit::from_vec(vec![
        (BaseUnit::KILOGRAM, 1),
        (BaseUnit::METER, 2),
        (BaseUnit::SECOND, -3),
    ])
}

/// Utility method to get the Volt Unit (composed)
pub fn get_volt() -> Unit {
    &get_watt() * &get_ampere().get_inverse()
}

/// Utility method to get the Ohm Unit (composed)
pub fn get_ohm() -> Unit {
    &get_volt() / &get_ampere()
}

/// Utility method to get the Siemens Unit (composed)
pub fn get_siemens() -> Unit {
    get_ohm().get_inverse()
}

/// Utility metgod to get the Pascal Unit (composed)
pub fn get_pascal() -> Unit {
    Unit::from_vec(vec![
        (BaseUnit::KILOGRAM, 1),
        (BaseUnit::METER, -1),
        (BaseUnit::SECOND, -2),
    ])
}

/// Utility method to get the Coulomb Unit (composed)
pub fn get_coulomb() -> Unit {
    &get_ampere() * &get_second()
}

/// Utility method to get the Coulomb Unit (composed)
pub fn get_farad() -> Unit {
    &get_coulomb() / &get_volt()
}

/// Utility method to get the Henry Unit (composed)
pub fn get_henry() -> Unit {
    Unit::from_vec(vec![
        (BaseUnit::KILOGRAM, 1),
        (BaseUnit::METER, 2),
        (BaseUnit::SECOND, -2),
        (BaseUnit::AMPERE, -2),
    ])
}

/// Utility method to get the Weber Unit (composed)
pub fn get_weber() -> Unit {
    &get_volt() * &get_second()
}

/// Utility method to get the becquerel Unit (composed)
pub fn get_becquerel() -> Unit {
    get_second().get_inverse()
}

/// Utility method to get the Hertz Unit (composed)
pub fn get_hertz() -> Unit {
    get_second().get_inverse()
}

/// Utility method to get the Tesla Unit (composed)
pub fn get_tesla() -> Unit {
    Unit::from_vec(vec![
        (BaseUnit::KILOGRAM, 1),
        (BaseUnit::SECOND, -2),
        (BaseUnit::AMPERE, -1),
    ])
}

#[cfg(test)]
mod test;
