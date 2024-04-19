//! ODE drawstuff bindings for Rust
//! drawstuff integrates bindings to cppbridge and cdrawstuff
//!
//! - cc-rs https://crates.io/crates/cc
//! - bindgen https://crates.io/crates/bindgen
//!
//! # cc-rs
//!
//! - include/bridge.hpp
//! - src/bridge.cpp
//!
//! # bindgen
//!
//! from
//!
//!  - include/bridge.hpp
//!  - drawstuff/drawstuff.h (from modified preprocess -E dum.cpp includes drawstuff.h)
//!
//! to
//!
//!  - include/bridge_bindings.rs
//!  - drawstuff/drawstuff_bindings.rs
//!
//! # Requirements
//!
//! in the running directory
//!
//! - drawstuff.dll
//! - libgcc_s_seh-1.dll
//! - libwinpthread-1.dll
//!

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cppbridge;
use cppbridge::*;
pub use cppbridge::{Bridge, bput};
pub use cppbridge::{dMULTIPLY0_331, dMULTIPLY0_333};
pub use cppbridge::{convexfvp, trimeshvi};
pub use cppbridge::{RecalcFaces, Normal4, Cross3};

mod cdrawstuff;
use cdrawstuff::*;

pub type dReal = f64; // replace bridge.hpp (defined in ode.hpp)
pub type dTriIndex = u32; // replace bridge.hpp (defined in ode.hpp)
// pub use cppode::{dMatrix4, dMatrix3, dVector4, dVector3}; // 16 12 4
// pub use cppode::{dQuaternion};

#[warn(unused)]
// #[warn(unused_imports)]
// #[warn(unused_attributes)]
#[warn(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(non_upper_case_globals)]

/// u32 RGBA (little endian) to dVector4 color
/// (not use dVector4 and mat::v2a to be independent of ode)
pub fn vec4_from_u32(col: u32) -> [dReal; 4] {
  let p: usize = &col as *const u32 as usize;
  (0..4).into_iter().map(|j|
unsafe {
    *((p + (3 - j)) as *const u8) as dReal / 255.0 // little endian
}
  ).collect::<Vec<_>>().try_into().unwrap()
}
