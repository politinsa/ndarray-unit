//! This module provides a struct being a simple representation of the international measure system and an Enum containing the seven base unit of the Internation SI plus other (radian, currency, pop...).
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

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Div, Mul};

/// An enum representing the seven units of the **International System of Units**
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BaseUnit {
    METER,
    SECOND,
    KILOGRAM,
    AMPERE,
    KELVIN,
    MOLE,
    CANDELA,

    // others useful
    RADIAN,
    STERADIAN,

    // economics indicator
    CURRENCY,
    INHABITANT,
    BIRTH,
    DEATH,
}

#[derive(Debug, Clone)]
pub struct Unit {
    /// A HashMap containing the base units of the Unit as well as their power
    base_units: HashMap<BaseUnit, i32>,
}

impl Unit {
    /// Return an Unit with an empty set of `BaseUnit`
    pub fn new() -> Unit {
        Unit {
            base_units: HashMap::new(),
        }
    }

    /// Create a Unit from a vector of BaseUnit and their power
    /// # Examples
    ///
    /// ```
    /// use ndarray_unit::Unit;
    /// use ndarray_unit::BaseUnit;
    ///
    /// let candela_square_divided_by_ampere_cube = Unit::from_vec(vec![(BaseUnit::CANDELA, 2), (BaseUnit::AMPERE, -3)]);
    /// println!("{}", candela_square_divided_by_ampere_cube)
    /// ```
    /// **Output**
    /// ```
    /// // cd²·A⁻³
    /// ```

    pub fn from_vec(list: Vec<(BaseUnit, i32)>) -> Unit {
        let mut res = Unit::new();
        for (unit, count) in list {
            res.add_single_unit(unit, count);
        }
        res
    }

    pub fn get_base_units(&self) -> &HashMap<BaseUnit, i32> {
        &self.base_units
    }

    /// Given an `Unit`, return the inverse of this unit.
    /// It inverse the sign of the power of every BaseUnit of the given input Struct
    ///
    /// # Exemples
    ///
    /// ```
    /// use ndarray_unit::Unit;
    /// use ndarray_unit::BaseUnit;
    ///
    /// let unit = Unit::from_vec(vec![(BaseUnit::KELVIN, 2), (BaseUnit::MOLE, -3)]);
    /// let inverse = Unit::from_vec(vec![(BaseUnit::KELVIN, -2), (BaseUnit::MOLE, 3)]);
    ///
    /// assert_eq!(inverse, unit.get_inverse());
    /// ```
    pub fn get_inverse(&self) -> Unit {
        let mut hashmap: HashMap<BaseUnit, i32> = HashMap::new();
        for (unit, count) in self.base_units.iter() {
            hashmap.insert(*unit, -count);
        }
        Unit {
            base_units: hashmap,
        }
    }

    /// Add a BaseUnit (and its power) to an existing `mut Unit`
    /// # Examples
    ///
    /// ```
    /// use ndarray_unit::Unit;
    /// use ndarray_unit::BaseUnit;
    ///
    /// let mut meter = Unit::new();
    /// meter.add_single_unit(BaseUnit::METER, 1);
    ///
    /// let mut meter_per_second = meter.clone();
    /// meter_per_second.add_single_unit(BaseUnit::SECOND, -1);
    /// println!("{}", meter_per_second);
    /// ```
    ///
    /// **Output**
    /// ```
    /// // m·s⁻¹
    /// ```
    pub fn add_single_unit(&mut self, unit: BaseUnit, n: i32) {
        let count = self.base_units.entry(unit).or_insert(0);
        *count += n;
    }

    fn add_from_hashmap(&mut self, hashmap: &HashMap<BaseUnit, i32>) {
        for (unit, count) in hashmap.iter() {
            self.add_single_unit(*unit, *count);
        }
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Unit) -> bool {
        let mut equal = true;
        for (unit, count) in self.base_units.iter() {
            if *count != 0 {
                match other.base_units.get(unit) {
                    Some(i) => {
                        if i != count {
                            equal = false;
                        }
                    }
                    _ => equal = false,
                }
            }
        }

        for (unit, count) in other.base_units.iter() {
            if *count != 0 {
                match self.base_units.get(unit) {
                    Some(i) => {
                        if i != count {
                            equal = false;
                        }
                    }
                    _ => equal = false,
                }
            }
        }
        equal
    }
}

/// Given two `Unit`s, perform a multiplication between them and return a new `Unit`.
/// It adds the base_units of the two given Struct
///
/// # Exemples
///
/// ```
/// use ndarray_unit::Unit;
/// use ndarray_unit::BaseUnit;
///
/// let meter = Unit::from_vec(vec![(BaseUnit::METER, 1)]);
/// let square_meter = Unit::from_vec(vec![(BaseUnit::METER, 2)]);
///
/// let cube_meter = &meter * &square_meter;
/// // cube_meter = Unit { base_units: {METER : 3} }
/// ```
impl Mul for &Unit {
    type Output = Unit;

    fn mul(self, other: &Unit) -> Unit {
        let mut result = Unit::new();
        result.add_from_hashmap(&self.base_units);
        result.add_from_hashmap(&other.base_units);
        result
    }
}

/// Given two `Unit`s, perform a division between them and return a new `Unit`.
/// It substracts the base_units of the two given Struct
///
/// # Exemples
///
/// ```
/// use ndarray_unit::Unit;
/// use ndarray_unit::BaseUnit;
///
/// let meter = Unit::from_vec(vec![(BaseUnit::METER, 1)]);
/// let second_square = Unit::from_vec(vec![(BaseUnit::SECOND, 2)]);
///
/// let acceleration = &meter / &second_square;
/// // acceleration = Unit { base_units: {METER : 1, SECOND: -2} }
/// ```
impl Div for &Unit {
    type Output = Unit;

    fn div(self, other: &Unit) -> Unit {
        let mut result = Unit::new();
        let inversed_unit = &other.get_inverse();
        let inversed_map = inversed_unit.get_base_units();
        result.add_from_hashmap(&self.base_units);
        result.add_from_hashmap(&inversed_map);
        result
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut list_units: Vec<(&BaseUnit, &i32)> = self
            .base_units
            .iter()
            .filter(|(_, val)| **val != 0)
            .collect();
        list_units.sort_by(|(_, val1), (_, val2)| val2.partial_cmp(val1).unwrap());

        if list_units.len() == 0 {
            return write!(f, "∅");
        }

        let mut res = String::new();

        let mut iterator = list_units.iter().peekable();
        while let Some((unit, count)) = iterator.next() {
            res.push_str(match unit {
                // SI
                BaseUnit::METER => "m",
                BaseUnit::SECOND => "s",
                BaseUnit::KILOGRAM => "kg",
                BaseUnit::AMPERE => "A",
                BaseUnit::KELVIN => "K",
                BaseUnit::MOLE => "mol",
                BaseUnit::CANDELA => "cd",
                // others
                BaseUnit::RADIAN => "rad",
                BaseUnit::STERADIAN => "sr",
                // eco
                BaseUnit::CURRENCY => "currency",
                BaseUnit::INHABITANT => "inhabitant",
                BaseUnit::BIRTH => "birth",
                BaseUnit::DEATH => "death",
            });

            if **count != 1 {
                let count_string = count.to_string();
                for c in count_string.chars() {
                    match c {
                        '-' => res.push_str("⁻"),
                        '0' => res.push_str("⁰"),
                        '1' => res.push_str("¹"),
                        '2' => res.push_str("²"),
                        '3' => res.push_str("³"),
                        '4' => res.push_str("⁴"),
                        '5' => res.push_str("⁵"),
                        '6' => res.push_str("⁶"),
                        '7' => res.push_str("⁷"),
                        '8' => res.push_str("⁸"),
                        '9' => res.push_str("⁹"),
                        _ => (),
                    }
                }
            }

            match iterator.peek() {
                Some(_) => res.push_str("·"),
                None => (),
            }
        }

        write!(f, "{}", res)
    }
}
