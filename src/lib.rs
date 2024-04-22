#![doc(html_root_url = "https://docs.rs/drawstuff/0.1.11")]
//! ODE drawstuff bindings for Rust
//!
//! # Requirements
//!
//! - [ https://github.com/nomissbowling/asciiz ]( https://github.com/nomissbowling/asciiz )
//! - [ drawstuff ]( https://ode.org/ )
//!
//! to build dll
//!
//! - premake4 --with-demos --only-double --with-libccd --cc=gcc --platform--x64 --os=windows codeblocks
//! - premake4 --with-demos --only-double --with-libccd --platform--x64 --os=windows vs2010
//!
//! in the running directory
//!
//! - drawstuff.dll
//! - libgcc_s_seh-1.dll
//! - libwinpthread-1.dll
//!
//! # Examples
//!
//! see also
//!
//! - [ https://github.com/nomissbowling/drawstuff ]( https://github.com/nomissbowling/drawstuff )
//!

pub mod drawstuff;

pub mod minimum;

#[cfg(test)]
mod tests {
  use super::drawstuff::{dReal, _dDot};
  use super::minimum::simple_test;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_drawstuff() {
    assert_eq!(simple_test(), ());
  }

  #[test]
  fn test_bridge() {
    let a: [dReal; 4] = [0.1, 0.2, 0.3, 0.4];
    let b: [dReal; 4] = [0.1, 0.2, 0.3, 0.4];
    let d = unsafe { _dDot(
      &a as *const [dReal; 4] as *const dReal,
      &b as *const [dReal; 4] as *const dReal,
      4) };
    assert_eq!(d - 0.3 < 0.000001, true);
  }

/*
  #[test]
  fn check_quaternion_matrix() {
    let q = dQuaternion::from_axis_and_angle([1.0, 0.0, 0.0], PIh);
    let p = dQuaternion::from_axis_and_angle([0.0, 1.0, 0.0], PIh);
    let o = dQuaternion::from_axis_and_angle([0.0, 0.0, 1.0], PIh);
    println!("q, p, o");
    println!("{}", q.as_vec());
    println!("{}", p.as_vec());
    println!("{}", o.as_vec());
    assert!(q.prec_eq(1e-15, [PIq.cos(), PIq.sin(), 0.0, 0.0]));
    assert!(p.prec_eq(1e-15, [PIq.cos(), 0.0, PIq.sin(), 0.0]));
    assert!(o.prec_eq(1e-15, [PIq.cos(), 0.0, 0.0, PIq.sin()]));

    let oq = dQuaternion::multiply0(o, q);
    let po = dQuaternion::multiply0(p, o);
    let pq = dQuaternion::multiply0(p, q);
    println!("oq, po, pq");
    println!("{}", oq.as_vec());
    println!("{}", po.as_vec());
    println!("{}", pq.as_vec());
    assert!(oq.prec_eq(1e-15, [0.5, 0.5, 0.5, 0.5])); // j
    assert!(po.prec_eq(1e-15, [0.5, 0.5, 0.5, 0.5])); // i
    assert!(pq.prec_eq(1e-15, [0.5, 0.5, 0.5, -0.5])); // k
    assert!(oq.prec_eq(1e-15, po));

    let nq = dMatrix4::from_P(q);
    let np = dMatrix4::from_P(p);
    let no = dMatrix4::from_P(o);
    println!("nq, np, no");
    println!("{}", nq.as_mat());
    println!("{}", np.as_mat());
    println!("{}", no.as_mat());
    assert!(dQuaternion::multiply0_441(nq, o).prec_eq(1e-15, oq));
    assert!(dQuaternion::multiply0_441(no, p).prec_eq(1e-15, po));
    assert!(dQuaternion::multiply0_441(nq, p).prec_eq(1e-15, pq));

    let nqno = dMatrix4::multiply0_444(nq, no);
    let nonp = dMatrix4::multiply0_444(no, np);
    let nqnp = dMatrix4::multiply0_444(nq, np);
    println!("nqno, nonp, nqnp");
    println!("{}", nqno.as_mat());
    println!("{}", nonp.as_mat());
    println!("{}", nqnp.as_mat());
    assert!(nqno.is_quaternion());
    assert!(nqno.to_Q().prec_eq(1e-15, oq));
    assert!(nonp.is_quaternion());
    assert!(nonp.to_Q().prec_eq(1e-15, po));
    assert!(nqnp.is_quaternion());
    assert!(nqnp.to_Q().prec_eq(1e-15, pq));
    assert!(nqno.prec_eq(1e-15, nonp));

    let mq = dMatrix4::from_Q(q);
    let mp = dMatrix4::from_Q(p);
    let mo = dMatrix4::from_Q(o);
    println!("mq, mp, mo");
    println!("{}", mq.as_mat());
    println!("{}", mp.as_mat());
    println!("{}", mo.as_mat());
    assert!(dQuaternion::multiply0_441(mo, q).prec_eq(1e-15, oq));
    assert!(dQuaternion::multiply0_441(mp, o).prec_eq(1e-15, po));
    assert!(dQuaternion::multiply0_441(mp, q).prec_eq(1e-15, pq));

    let momq = dMatrix4::multiply0_444(mo, mq);
    let mpmo = dMatrix4::multiply0_444(mp, mo);
    let mpmq = dMatrix4::multiply0_444(mp, mq);
    println!("momq, mpmo, mpmq");
    println!("{}", momq.as_mat());
    println!("{}", mpmo.as_mat());
    println!("{}", mpmq.as_mat());
    assert!(momq.is_quaternion());
    assert!(momq.to_Q().prec_eq(1e-15, oq));
    assert!(mpmo.is_quaternion());
    assert!(mpmo.to_Q().prec_eq(1e-15, po));
    assert!(mpmq.is_quaternion());
    assert!(mpmq.to_Q().prec_eq(1e-15, pq));
    assert!(momq.prec_eq(1e-15, mpmo));
  }
*/
}

/*
q, p, o
[        0.7071068,        0.7071068,        0.0000000,        0.0000000]
[        0.7071068,        0.0000000,        0.7071068,        0.0000000]
[        0.7071068,        0.0000000,        0.0000000,        0.7071068]
oq, po, pq
[        0.5000000,        0.5000000,        0.5000000,        0.5000000]
[        0.5000000,        0.5000000,        0.5000000,        0.5000000]
[        0.5000000,        0.5000000,        0.5000000,       -0.5000000]
nq, np, no
[
 [        0.7071068,       -0.7071068,       -0.0000000,       -0.0000000]
 [        0.7071068,        0.7071068,        0.0000000,       -0.0000000]
 [        0.0000000,       -0.0000000,        0.7071068,        0.7071068]
 [        0.0000000,        0.0000000,       -0.7071068,        0.7071068]]
[
 [        0.7071068,       -0.0000000,       -0.7071068,       -0.0000000]
 [        0.0000000,        0.7071068,        0.0000000,       -0.7071068]
 [        0.7071068,       -0.0000000,        0.7071068,        0.0000000]
 [        0.0000000,        0.7071068,       -0.0000000,        0.7071068]]
[
 [        0.7071068,       -0.0000000,       -0.0000000,       -0.7071068]
 [        0.0000000,        0.7071068,        0.7071068,       -0.0000000]
 [        0.0000000,       -0.7071068,        0.7071068,        0.0000000]
 [        0.7071068,        0.0000000,       -0.0000000,        0.7071068]]
nqno, nonp, nqnp
[
 [        0.5000000,       -0.5000000,       -0.5000000,       -0.5000000]
 [        0.5000000,        0.5000000,        0.5000000,       -0.5000000]
 [        0.5000000,       -0.5000000,        0.5000000,        0.5000000]
 [        0.5000000,        0.5000000,       -0.5000000,        0.5000000]]
[
 [        0.5000000,       -0.5000000,       -0.5000000,       -0.5000000]
 [        0.5000000,        0.5000000,        0.5000000,       -0.5000000]
 [        0.5000000,       -0.5000000,        0.5000000,        0.5000000]
 [        0.5000000,        0.5000000,       -0.5000000,        0.5000000]]
[
 [        0.5000000,       -0.5000000,       -0.5000000,        0.5000000]
 [        0.5000000,        0.5000000,       -0.5000000,       -0.5000000]
 [        0.5000000,        0.5000000,        0.5000000,        0.5000000]
 [       -0.5000000,        0.5000000,       -0.5000000,        0.5000000]]
mq, mp, mo
[
 [        0.7071068,       -0.7071068,       -0.0000000,       -0.0000000]
 [        0.7071068,        0.7071068,       -0.0000000,        0.0000000]
 [        0.0000000,        0.0000000,        0.7071068,       -0.7071068]
 [        0.0000000,       -0.0000000,        0.7071068,        0.7071068]]
[
 [        0.7071068,       -0.0000000,       -0.7071068,       -0.0000000]
 [        0.0000000,        0.7071068,       -0.0000000,        0.7071068]
 [        0.7071068,        0.0000000,        0.7071068,       -0.0000000]
 [        0.0000000,       -0.7071068,        0.0000000,        0.7071068]]
[
 [        0.7071068,       -0.0000000,       -0.0000000,       -0.7071068]
 [        0.0000000,        0.7071068,       -0.7071068,        0.0000000]
 [        0.0000000,        0.7071068,        0.7071068,       -0.0000000]
 [        0.7071068,       -0.0000000,        0.0000000,        0.7071068]]
momq, mpmo, mpmq
[
 [        0.5000000,       -0.5000000,       -0.5000000,       -0.5000000]
 [        0.5000000,        0.5000000,       -0.5000000,        0.5000000]
 [        0.5000000,        0.5000000,        0.5000000,       -0.5000000]
 [        0.5000000,       -0.5000000,        0.5000000,        0.5000000]]
[
 [        0.5000000,       -0.5000000,       -0.5000000,       -0.5000000]
 [        0.5000000,        0.5000000,       -0.5000000,        0.5000000]
 [        0.5000000,        0.5000000,        0.5000000,       -0.5000000]
 [        0.5000000,       -0.5000000,        0.5000000,        0.5000000]]
[
 [        0.5000000,       -0.5000000,       -0.5000000,        0.5000000]
 [        0.5000000,        0.5000000,        0.5000000,        0.5000000]
 [        0.5000000,       -0.5000000,        0.5000000,       -0.5000000]
 [       -0.5000000,       -0.5000000,        0.5000000,        0.5000000]]
*/
