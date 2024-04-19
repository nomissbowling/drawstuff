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
include!(concat!("../include", "/bridge_bindings.rs"));

// use above privates only for minimum test (MUST NOT use include in your code)

use crate::drawstuff::*;

use asciiz::u8z::{U8zBuf, u8zz::{CArgsBuf}};

use std::pin::Pin;
use std::sync::Arc;
use std::cell::RefCell;
use std::borrow::BorrowMut;

/// trait TBridgeGlobal
pub trait TBridgeGlobal {
  /// constructor
  fn new() -> Self;
}

/// TBridgeGlobal for bridgeGlobal
impl TBridgeGlobal for bridgeGlobal {
  /// constructor
  fn new() -> Self {
    bridgeGlobal{num: 0, buf: [0; 8]}
  }
}

/// fake with_bg_mut for any pinned type
#[macro_export]
macro_rules! any_pinned_with_bg_mut {
  ($i: ident, $n: expr, $f: expr) => {
    Pin::<&mut Arc<RefCell<$i>>>::with_bg_mut($n, $f)
  }
}

/// trait TGlobalSetGet
pub trait TGlobalSetGet<T> {
  /// set bg
  fn set_bg_mut(&mut self, n: usize);
  /// with bg
  fn with_bg_mut<F>(n: usize, f: F) where F: Fn(&mut T) -> ();
}

/// TGlobalSetGet for Pin<&mut Arc<RefCell<T>>>
impl<T> TGlobalSetGet<T> for Pin<&mut Arc<RefCell<T>>> {
  /// set bg
  fn set_bg_mut(&mut self, n: usize) {
    unsafe {
      match bridge_global_setter(n, self.borrow_mut().as_ptr()
        as *mut T as *mut bridgeGlobal) {
        0 => panic!("not allocated area: bridge_global_setter"),
        _ => ()
      }
    }
  }
  /// with bg
  fn with_bg_mut<F>(n: usize, f: F) where F: Fn(&mut T) -> () {
    f(unsafe {
      let p = bridge_global_getter(n);
      if p == 0 as *mut bridgeGlobal {
        panic!("not allocated area: bridge_global_getter");
      }
      &mut std::slice::from_raw_parts_mut(p as *mut T, 1)[0] // fake
    })
  }
}

/// start callback
#[no_mangle]
pub extern "C" fn c_start_callback() {
  println!("c_start_callback");
  any_pinned_with_bg_mut!(bridgeGlobal, 0, |bg| {
    bg.num = 0;
  });
}

/// step callback
#[no_mangle]
pub extern "C" fn c_step_callback(pause: i32) {
  any_pinned_with_bg_mut!(bridgeGlobal, 0, move |bg| {
    if pause != bg.num as i32 {
      bg.num = pause as usize;
      println!("c_step_callback: pause = {}", pause);
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
  let mut abg = Arc::new(RefCell::new(bridgeGlobal::new()));
  let mut pbg = Pin::new(&mut abg); // to keep lifetime
  pbg.set_bg_mut(0);

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
}
