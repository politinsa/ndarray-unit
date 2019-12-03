//! This module provides a struct being a simple representation of the international measure system and an Enum containing all of the seven base unit of the Internation SI.    
//! It allows to multiply and divise unit between them and to print them.  
//! Several utility functions are provided to construct your own derived Unit from the base ones
//!
//! # Exemples
//!
//! ```
//! use ndarray_unit;
//! use ndarray_unit::{Unit, BaseUnit};
//!
//! let meter_per_sec = Unit::from_vec(vec![(BaseUnit::METER, 1), (BaseUnit::SECOND, -1)]);
//! println!("meter_per_sec = {}", meter_per_sec);
//!
//! let acceleration = &meter_per_sec / &ndarray_unit::get_second();
//! println!("acceleration = {}", acceleration);
//! ```
//! **Output**
//! ```
//! // meter_per_sec = m·s⁻¹
//! // acceleration = m·s⁻²
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

/// Utility method to get a Unit from a BaseUnit (BaseUnit::RADIAN)
pub fn get_radian() -> Unit {
    Unit::from_vec(vec![(BaseUnit::RADIAN, 1)])
}

/// Utility method to get a Unit from a BaseUnit (BaseUnit::STERADIAN)
pub fn get_steradian() -> Unit {
    Unit::from_vec(vec![(BaseUnit::STERADIAN, 1)])
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
