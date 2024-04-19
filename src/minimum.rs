//! minimum test for drawstuff
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

include!(concat!("../drawstuff", "/drawstuff_bindings.rs"));
include!(concat!("../include", "/bridge_bindings.rs")); // test for _dDot

// use above privates only for minimum test (MUST NOT use include in your code)

use crate::drawstuff::*;

use asciiz::u8z::{U8zBuf, u8zz::{CArgsBuf}};

use anyslot::anyslot::{
  bridge_global_init_slots, bridge_global_dispose_slots,
  any_pinned_init_slots, any_pinned_dispose_slots,
  any_pinned_set_bg_mut, any_pinned_with_bg_mut,
  TBridgeGlobal, TGlobalSetGet, bridge_global};

use std::pin::Pin;
use std::sync::Arc;
use std::cell::RefCell;
use std::borrow::BorrowMut;

/// AnySlot
#[derive(Debug, Clone)]
pub struct AnySlot {
  /// r
  pub r: u8,
  /// g
  pub g: u8,
  /// b
  pub b: u8,
  /// a
  pub a: u8
}

/// TBridgeGlobal for AnySlot
impl TBridgeGlobal for AnySlot {
  /// constructor
  fn void() -> Self {
    AnySlot::new(0, 0, 0, 0)
  }
}

/// AnySlot
impl AnySlot {
  /// constructor
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    AnySlot{r, g, b, a}
  }
}

/// start callback
#[no_mangle]
pub extern "C" fn c_start_callback() {
  println!("c_start_callback");
  any_pinned_with_bg_mut!(bridge_global, 0, |bg| {
    bg.num = 0;
  });
  any_pinned_with_bg_mut!(AnySlot, 1, |bg| {
    println!("{:?}", bg);
    *bg = AnySlot::new(240, 192, 32, 255);
    println!("{:?}", bg);
  });
}

/// step callback
#[no_mangle]
pub extern "C" fn c_step_callback(pause: i32) {
  any_pinned_with_bg_mut!(bridge_global, 0, move |bg| { // test with move
    if pause != bg.num as i32 {
      bg.num = pause as usize;
      println!("c_step_callback: pause = {}", pause);

      any_pinned_with_bg_mut!(AnySlot, 1, |bg| { // another slot in the closure
        println!("{:?}", bg);
      });
    }
  });
}

/// command callback
#[no_mangle]
pub extern "C" fn c_command_callback(cmd: i32) {
  println!("c_command_callback: cmd = {}", cmd);
}

/// stop callback
#[no_mangle]
pub extern "C" fn c_stop_callback() {
  println!("c_stop_callback");
}

/// simple test
pub fn simple_test() {
  any_pinned_init_slots!(8);
  any_pinned_set_bg_mut!(bridge_global, 0);
  any_pinned_set_bg_mut!(AnySlot, 1);

  let width = 800i32;
  let height = 600i32;
  let a: &[u8] = b"./resources";
  let ptt = &Some(U8zBuf::from_u8array(a)); // to keep lifetime
  let mut dsfn = dsFunctions{
    version: DS_VERSION,
    start: Some(c_start_callback), // Option<unsafe extern "C" fn()>
    step: Some(c_step_callback), // Option<unsafe extern "C" fn(i32)>
    command: Some(c_command_callback), // Option<unsafe extern "C" fn(i32)>
    stop: Some(c_stop_callback), // Option<unsafe extern "C" fn()>
    path_to_textures: ptt.as_ref().expect("not init").as_i8p()
  };
  let mut cab: CArgsBuf = CArgsBuf::from(&std::env::args().collect());
unsafe {
  dsSimulationLoop(cab.as_argc(), cab.as_argv_ptr_mut(),
    width, height, &mut dsfn);
}

  any_pinned_dispose_slots!();
}
