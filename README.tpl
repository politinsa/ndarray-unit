[![Build Status - Travis](https://travis-ci.com/politinsa/ndarray-unit.svg?branch=master)](https://travis-ci.com/politinsa/ndarray-unit)
![Build Status - Github](https://github.com/politinsa/ndarray-unit/workflows/Build/badge.svg)
![Tests - Github](https://github.com/politinsa/ndarray-unit/workflows/Tests/badge.svg)
[![crates.io](https://meritbadge.herokuapp.com/ndarray-unit)](https://crates.io/crates/ndarray-unit)
[![docs.rs](https://docs.rs/ndarray-unit/badge.svg)](https://docs.rs/ndarray-unit/)

# ndarray-unit

* Homepage on [crates.io](https://crates.io/crates/ndarray-unit)
* **Doc** on [doc.rs](https://docs.rs/ndarray-unit/)  

-----

:warning: This project is under active development and this README is using
documentation comments that might be different than those uploaded on crates.io.
Always check [doc.rs](https://docs.rs/ndarray-unit/) for up-to-date
documentation. :warning:  

-----

{{readme}}


## Development
Doc of the master branch on [github.io](https://politinsa.github.io/ndarray-unit/)

### Features

- [x] Basic unit system handling multiplication and division
- [x] ArrayUnit wrapper for unit + ndarray
- [ ] Operations on &ArrayUnit
     - [x] Mul/Div <&ArrayUnit>
     - [x] Add/Sub <&ArrayUnit> (now panicking if units are not equal, might change that in the future)
     - [ ] Mul/Div with scalar
     - [ ] Add/Sub with scalar
- [ ] Add prefix (mm, cm, dm, m, km)
- [ ] Basic ndarray function
     - [ ] Transpose
     - [ ] Rescale
     - [ ] ...



### Tests & Doc
- [x] Basic unit system handling multiplication and division
- [x] ArrayUnit wrapper for unit + ndarray
- [ ] Operations on &ArrayUnit
     - [ ] Mul/Div <&ArrayUnit>
     - [ ] Add/Sub <&ArrayUnit> (now panicking if units are not equal, might change that in the future)
     - [ ] Mul/Div with scalar
     - [ ] Add/Sub with scalar
- [ ] Add prefix (mm, cm, dm, m, km)
- [ ] Basic ndarray function
     - [ ] Transpose
     - [ ] Rescale
     - [ ] ...
