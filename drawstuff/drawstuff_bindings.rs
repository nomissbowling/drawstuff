/* automatically generated by rust-bindgen 0.65.1 */

pub const DS_VERSION: ::std::os::raw::c_int = 2;
pub const DS_TEXTURE_NUMBER_DS_NONE: DS_TEXTURE_NUMBER = 0;
pub const DS_TEXTURE_NUMBER_DS_WOOD: DS_TEXTURE_NUMBER = 1;
pub const DS_TEXTURE_NUMBER_DS_CHECKERED: DS_TEXTURE_NUMBER = 2;
pub const DS_TEXTURE_NUMBER_DS_GROUND: DS_TEXTURE_NUMBER = 3;
pub const DS_TEXTURE_NUMBER_DS_SKY: DS_TEXTURE_NUMBER = 4;
pub type DS_TEXTURE_NUMBER = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dsFunctions {
    pub version: ::std::os::raw::c_int,
    pub start: ::std::option::Option<unsafe extern "C" fn()>,
    pub step: ::std::option::Option<unsafe extern "C" fn(pause: ::std::os::raw::c_int)>,
    pub command: ::std::option::Option<unsafe extern "C" fn(cmd: ::std::os::raw::c_int)>,
    pub stop: ::std::option::Option<unsafe extern "C" fn()>,
    pub path_to_textures: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_dsFunctions() {
    const UNINIT: ::std::mem::MaybeUninit<dsFunctions> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<dsFunctions>(),
        48usize,
        concat!("Size of: ", stringify!(dsFunctions))
    );
    assert_eq!(
        ::std::mem::align_of::<dsFunctions>(),
        8usize,
        concat!("Alignment of ", stringify!(dsFunctions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dsFunctions),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).start) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(dsFunctions),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).step) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(dsFunctions),
            "::",
            stringify!(step)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).command) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(dsFunctions),
            "::",
            stringify!(command)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(dsFunctions),
            "::",
            stringify!(stop)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).path_to_textures) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(dsFunctions),
            "::",
            stringify!(path_to_textures)
        )
    );
}
extern "C" {
    pub fn dsDebug(msg: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn dsDrawBox(pos: *const f32, R: *const f32, sides: *const f32);
}
extern "C" {
    pub fn dsDrawBoxD(pos: *const f64, R: *const f64, sides: *const f64);
}
extern "C" {
    pub fn dsDrawCapsule(pos: *const f32, R: *const f32, length: f32, radius: f32);
}
extern "C" {
    pub fn dsDrawCapsuleD(pos: *const f64, R: *const f64, length: f32, radius: f32);
}
extern "C" {
    pub fn dsDrawConvex(
        pos: *const f32,
        R: *const f32,
        _planes: *const f32,
        _planecount: ::std::os::raw::c_uint,
        _points: *const f32,
        _pointcount: ::std::os::raw::c_uint,
        _polygons: *const ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn dsDrawConvexD(
        pos: *const f64,
        R: *const f64,
        _planes: *const f64,
        _planecount: ::std::os::raw::c_uint,
        _points: *const f64,
        _pointcount: ::std::os::raw::c_uint,
        _polygons: *const ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn dsDrawCylinder(pos: *const f32, R: *const f32, length: f32, radius: f32);
}
extern "C" {
    pub fn dsDrawCylinderD(pos: *const f64, R: *const f64, length: f32, radius: f32);
}
extern "C" {
    pub fn dsDrawLine(pos1: *const f32, pos2: *const f32);
}
extern "C" {
    pub fn dsDrawLineD(pos1: *const f64, pos2: *const f64);
}
extern "C" {
    pub fn dsDrawSphere(pos: *const f32, R: *const f32, radius: f32);
}
extern "C" {
    pub fn dsDrawSphereD(pos: *const f64, R: *const f64, radius: f32);
}
extern "C" {
    pub fn dsDrawTriangle(
        pos: *const f32,
        R: *const f32,
        v0: *const f32,
        v1: *const f32,
        v2: *const f32,
        solid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn dsDrawTriangleD(
        pos: *const f64,
        R: *const f64,
        v0: *const f64,
        v1: *const f64,
        v2: *const f64,
        solid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn dsDrawTriangles(
        pos: *const f32,
        R: *const f32,
        v: *const f32,
        n: ::std::os::raw::c_int,
        solid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn dsDrawTrianglesD(
        pos: *const f64,
        R: *const f64,
        v: *const f64,
        n: ::std::os::raw::c_int,
        solid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn dsElapsedTime() -> f64;
}
extern "C" {
    pub fn dsError(msg: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn dsGetViewpoint(xyz: *mut f32, hpr: *mut f32);
}
extern "C" {
    pub fn dsPrint(msg: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn dsSetCapsuleQuality(n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn dsSetColor(red: f32, green: f32, blue: f32);
}
extern "C" {
    pub fn dsSetColorAlpha(red: f32, green: f32, blue: f32, alpha: f32);
}
extern "C" {
    pub fn dsSetDrawMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn dsSetSphereQuality(n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn dsSetTexture(texture_number: ::std::os::raw::c_int);
}
extern "C" {
    pub fn dsSetViewpoint(xyz: *mut f32, hpr: *mut f32);
}
extern "C" {
    pub fn dsSimulationLoop(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        window_width: ::std::os::raw::c_int,
        window_height: ::std::os::raw::c_int,
        fn_: *mut dsFunctions,
    );
}
extern "C" {
    pub fn dsStop();
}