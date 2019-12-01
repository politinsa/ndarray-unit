use super::*;

#[test]
fn test_eq() {
    let empty = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 0),
        (BaseUnit::CANDELA, 0),
        (BaseUnit::KELVIN, 0),
        (BaseUnit::METER, 0),
        (BaseUnit::MOLE, 0),
    ]);
    let unit1 = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
    ]);
    let unit1bis = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
    ]);
    let unit2 = Unit::from_vec(vec![
        (BaseUnit::METER, 3),
        (BaseUnit::KELVIN, -12),
        (BaseUnit::MOLE, 8),
    ]);

    assert_eq!(Unit::new(), Unit::new());
    assert_eq!(Unit::new(), empty);
    assert_eq!(unit1, unit1bis);
    assert_ne!(unit1, unit2);
}

#[test]
fn test_multiply() {
    let unit1 = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
    ]);
    let unit2 = Unit::from_vec(vec![
        (BaseUnit::METER, 3),
        (BaseUnit::KELVIN, -12),
        (BaseUnit::MOLE, 8),
    ]);

    let result = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 10),
        (BaseUnit::METER, 3),
        (BaseUnit::MOLE, 8),
    ]);

    assert_eq!(result, &unit1 * &unit2);
    assert_eq!(result, &unit2 * &unit1);
}

#[test]
fn test_divise() {
    let unit1 = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
    ]);
    let unit2 = Unit::from_vec(vec![
        (BaseUnit::METER, 3),
        (BaseUnit::KELVIN, -12),
        (BaseUnit::MOLE, 8),
    ]);

    let result = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
        (BaseUnit::METER, -3),
        (BaseUnit::KELVIN, 12),
        (BaseUnit::MOLE, -8),
    ]);

    assert_eq!(result, &unit1 / &unit2);
    assert_eq!(Unit::new(), &unit1 / &unit1);
    assert_eq!(Unit::new(), &unit2 / &unit2);
}

#[test]
fn test_inverse() {
    let unit1 = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
    ]);
    let unit2 = Unit::from_vec(vec![
        (BaseUnit::METER, 3),
        (BaseUnit::KELVIN, -12),
        (BaseUnit::MOLE, 8),
    ]);

    // result = unit1/unit2
    let result = Unit::from_vec(vec![
        (BaseUnit::AMPERE, 10),
        (BaseUnit::CANDELA, -2),
        (BaseUnit::KELVIN, 22),
        (BaseUnit::METER, -3),
        (BaseUnit::KELVIN, 12),
        (BaseUnit::MOLE, -8),
    ]);

    assert_eq!(result, &unit1 * &unit2.get_inverse());
    assert_eq!(Unit::new(), &unit1 * &unit1.get_inverse());
}
