![](https://github.com/politinsa/ndarray-unit/workflows/Build/badge.svg)
![](https://github.com/politinsa/ndarray-unit/workflows/Tests/badge.svg)
# ndarray-unit

This module provides a struct being a simple representation of the international measure system and an Enum containing all of the seven base unit of the Internation SI.
It allows to multiply and divise unit between them and to print them.
Several utility functions are provided to construct your own derived Unit from the base ones

## Exemples

```rust
use ndarray_unit;
use ndarray_unit::{Unit, BaseUnit};

let meter_per_sec = Unit::from_vec(vec![(BaseUnit::METER, 1), (BaseUnit::SECOND, -1)]);
println!("meter_per_sec = {}", meter_per_sec);

let acceleration = &meter_per_sec / &ndarray_unit::get_second();
println!("acceleration = {}", acceleration);
```
**Output**
```rust
// meter_per_sec = m·s⁻¹
// acceleration = m·s⁻²
```

License: MIT OR Apache-2.0
