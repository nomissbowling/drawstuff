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

use num::Float;

/// Color4u8 to [F; 4]
pub fn vec4_from_4u8<F: Float + std::fmt::Debug>(col: &Color4u8) -> [F; 4] {
  [col.r, col.g, col.b, col.a].into_iter().map(|u|
    <F>::from(u).unwrap() / <F>::from(255.0).unwrap()
  ).collect::<Vec<_>>().try_into().unwrap()
}

/// COLORS
#[derive(Debug, Clone)]
pub struct COLORS {
  /// c
  pub c: [Color4u8; 16]
}

/// fmt::Display for COLORS
impl std::fmt::Display for COLORS {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[");
    for (i, c) in self.c.iter().enumerate() {
      if i != 0 { write!(f, " "); }
      write!(f, "{}", c);
      if i == self.c.len() - 1 { write!(f, "]"); } else { writeln!(f, ","); }
    }
    Ok(())
  }
}

/// TBridgeGlobal for COLORS
impl TBridgeGlobal for COLORS {
  /// constructor
  fn void() -> Self {
    let c = [
      0xcccccccc, 0xcc9933cc, 0x33cc99cc, 0x9933cccc,
      0x3399cccc, 0x99cc33cc, 0xcc3399cc, 0x999999cc,
      0x666666cc, 0x996633cc, 0x339966cc, 0x663399cc,
      0x336699cc, 0x669933cc, 0x993366cc, 0x333333cc].into_iter().map(|u|
      Color4u8::from_u32(u)).collect::<Vec<_>>().try_into().unwrap();
    COLORS{c}
  }
}

/// Color4u8
#[derive(Debug, Clone)]
pub struct Color4u8 {
  /// r
  pub r: u8,
  /// g
  pub g: u8,
  /// b
  pub b: u8,
  /// a
  pub a: u8
}

/// fmt::Display for Color4u8
impl std::fmt::Display for Color4u8 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[{:02x}, {:02x}, {:02x}, {:02x}]",
      self.r, self.g, self.b, self.a)
  }
}

/// Color4u8
impl Color4u8 {
  /// constructor
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color4u8{r, g, b, a}
  }
  /// constructor
  pub fn from_vec(v: &[u8]) -> Self {
    Color4u8::new(v[0], v[1], v[2], v[3])
  }
  /// constructor
  pub fn from_u32(col: u32) -> Self {
    let p: usize = &col as *const u32 as usize;
    let v = (0..4).into_iter().map(|j|
      unsafe { *((p + (3 - j)) as *const u8) } // little endian
    ).collect::<Vec<_>>();
    Color4u8::from_vec(&v)
  }
}

/// Stat
#[derive(Debug, Clone)]
pub struct Stat {
  /// tick
  pub tick: u32
}

/// TBridgeGlobal for Stat
impl TBridgeGlobal for Stat {
  /// constructor
  fn void() -> Self {
    Stat{tick: 0}
  }
}

/// draw
pub fn draw(st: &Stat, col: &[f32; 4]) {
unsafe {
  // dsSetDrawMode(DS_POLYFILL); // WIREFRAME
  // dsSetTexture(DS_WOOD); // CHECKERED GROUND SKY

  dsSetColorAlpha(col[0], col[1], col[2], col[3]);
//  let c = vec4_from_u32::<f32>(0xE0C020FF);
//  dsSetColorAlpha(c[0], c[1], c[2], c[3]);
  let r = 0.1f32;
  let p: [f32; 3] = [0.0, 0.0, r];
  let m: [f32; 12] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0];
  dsDrawSphere(
    &p as *const [f32; 3] as *const f32,
    &m as *const [f32; 12] as *const f32,
    r);

  dsSetTexture(DS_TEXTURE_NUMBER_DS_WOOD);
  let c = vec4_from_u32::<f32>(0xC0E020C0);
  dsSetColorAlpha(c[0], c[1], c[2], c[3]);
  let a = 15.0f32;
  let t = a * std::f32::consts::PI / 180.0f32;
  let (c, s) = (t.cos(), t.sin());
  let l = 0.5f32;
  let r = 0.1f32;
  let p: [f32; 3] = [0.5, 0.0, r + l / 2.0];
  let m: [f32; 12] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, c, -s, 0.0,
    0.0, s, c, 0.0];
  dsDrawCapsule(
    &p as *const [f32; 3] as *const f32,
    &m as *const [f32; 12] as *const f32,
    l,
    r);

  dsSetTexture(DS_TEXTURE_NUMBER_DS_CHECKERED);
  dsSetColorAlpha(col[0], col[1], col[2], col[3]);
//  let c = vec4_from_u32::<f32>(0xC020E0C0);
//  dsSetColorAlpha(c[0], c[1], c[2], c[3]);
  let a = ((st.tick / 6) % 360) as f32;
  let t = a * std::f32::consts::PI / 180.0f32;
  let (cp, sp) = (t.cos(), t.sin());
  let (c, s) = { let t = t * 6.0; (t.cos(), t.sin()) }; // change t
  let l = 0.5f32;
  let r = 0.1f32;
  let p: [f32; 3] = [cp, sp, r + l / 2.0];
  let m: [f32; 12] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, c, -s, 0.0,
    0.0, s, c, 0.0];
  dsDrawCylinder(
    &p as *const [f32; 3] as *const f32,
    &m as *const [f32; 12] as *const f32,
    l,
    r);
}
}

/// start callback
#[no_mangle]
pub extern "C" fn c_start_callback() {
  println!("c_start_callback");
  any_pinned_with_bg_mut!(bridge_global, 0, |bg| {
    bg.num = 0;
  });
  any_pinned_with_bg_mut!(COLORS, 1, |bg| {
    println!("{}", bg);
    bg.c[0] = Color4u8::new(240, 192, 32, 128); // test overwrite
    println!("{}", bg);
  });
  any_pinned_with_bg_mut!(Stat, 2, |st| {
    st.tick = 0;
  });

unsafe {
  dsSetSphereQuality(3); // default 1
  dsSetCapsuleQuality(3); // default 3
}
}

/// step callback
#[no_mangle]
pub extern "C" fn c_step_callback(pause: i32) {
  any_pinned_with_bg_mut!(bridge_global, 0, |bg| {
    if pause != bg.num as i32 {
      bg.num = pause as usize;
      println!("c_step_callback: pause = {}", pause);

      any_pinned_with_bg_mut!(COLORS, 1, |bg| { // another slot in the closure
        println!("{}", bg.c[0]);
      });

unsafe {
      let et = dsElapsedTime(); // f64
      println!("{:?}", et);
}
    }
  });

  any_pinned_with_bg_mut!(Stat, 2, |st| {
    let mut col = [0.0f32; 4];
    any_pinned_with_bg_mut!(COLORS, 1, |bg| {
      let n = (st.tick as usize / 1000) % bg.c.len();
      col = vec4_from_4u8::<f32>(&bg.c[n]);
    });
    draw(&st, &col);
    st.tick += 1;
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
  any_pinned_set_bg_mut!(COLORS, 1);
  any_pinned_set_bg_mut!(Stat, 2);

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
