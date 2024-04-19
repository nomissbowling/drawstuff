/* automatically generated by rust-bindgen 0.65.1 */

#[doc = " universal area bridge global"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bridgeGlobal {
    #[doc = " num"]
    pub num: usize,
    #[doc = " may be changed by alloc"]
    pub buf: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_bridgeGlobal() {
    const UNINIT: ::std::mem::MaybeUninit<bridgeGlobal> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bridgeGlobal>(),
        16usize,
        concat!("Size of: ", stringify!(bridgeGlobal))
    );
    assert_eq!(
        ::std::mem::align_of::<bridgeGlobal>(),
        8usize,
        concat!("Alignment of ", stringify!(bridgeGlobal))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).num) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bridgeGlobal),
            "::",
            stringify!(num)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buf) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bridgeGlobal),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    #[doc = " bridge global setter"]
    pub fn bridge_global_setter(n: usize, p: *mut bridgeGlobal) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " bridge global getter"]
    pub fn bridge_global_getter(n: usize) -> *mut bridgeGlobal;
}
#[repr(C)]
pub struct Bridge__bindgen_vtable(::std::os::raw::c_void);
#[doc = " Bridge for cpp"]
#[repr(C)]
#[derive(Debug)]
pub struct Bridge {
    pub vtable_: *const Bridge__bindgen_vtable,
    pub str_: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_Bridge() {
    const UNINIT: ::std::mem::MaybeUninit<Bridge> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Bridge>(),
        16usize,
        concat!("Size of: ", stringify!(Bridge))
    );
    assert_eq!(
        ::std::mem::align_of::<Bridge>(),
        8usize,
        concat!("Alignment of ", stringify!(Bridge))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Bridge),
            "::",
            stringify!(str_)
        )
    );
}
extern "C" {
    #[doc = " assign ptr"]
    #[link_name = "\u{1}?pset@Bridge@@QEAAXPEAD@Z"]
    pub fn Bridge_pset(this: *mut Bridge, p: *mut ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " display"]
    #[link_name = "\u{1}?put@Bridge@@QEAAXXZ"]
    pub fn Bridge_put(this: *mut Bridge);
}
extern "C" {
    #[doc = " construct"]
    #[link_name = "\u{1}??0Bridge@@QEAA@XZ"]
    pub fn Bridge_Bridge(this: *mut Bridge);
}
extern "C" {
    #[doc = " construct with ptr"]
    #[link_name = "\u{1}??0Bridge@@QEAA@PEAD@Z"]
    pub fn Bridge_Bridge1(this: *mut Bridge, s: *mut ::std::os::raw::c_char);
}
impl Bridge {
    #[inline]
    pub unsafe fn pset(&mut self, p: *mut ::std::os::raw::c_char) {
        Bridge_pset(self, p)
    }
    #[inline]
    pub unsafe fn put(&mut self) {
        Bridge_put(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Bridge_Bridge(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(s: *mut ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Bridge_Bridge1(__bindgen_tmp.as_mut_ptr(), s);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[doc = " destruct"]
    #[link_name = "\u{1}??_DBridge@@QEAAXXZ"]
    pub fn Bridge_Bridge_destructor(this: *mut Bridge);
}
extern "C" {
    #[doc = " legacy C interface"]
    pub fn bput();
}
#[doc = " dReal as f64 (defined in ode.hpp)"]
pub type dReal = f64;
#[doc = " dTriIndex as u32 (defined in ode.hpp)"]
pub type dTriIndex = ::std::os::raw::c_uint;
extern "C" {
    #[doc = " res = a(&dMatrix3) b(&dVector3)"]
    pub fn dMULTIPLY0_331(res: *mut dReal, a: *const dReal, b: *const dReal);
}
extern "C" {
    #[doc = " res = a(&dMatrix3) b(&dMatrix3)"]
    pub fn dMULTIPLY0_333(res: *mut dReal, a: *const dReal, b: *const dReal);
}
extern "C" {
    #[doc = " res = a(&dMatrix4) b(&dVector4 or &dQuaternion)"]
    pub fn dMULTIPLY0_441(res: *mut dReal, a: *const dReal, b: *const dReal);
}
extern "C" {
    #[doc = " res = a(&dMatrix4) b(&dMatrix4)"]
    pub fn dMULTIPLY0_444(res: *mut dReal, a: *const dReal, b: *const dReal);
}
#[doc = " TriMeshVI"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct trimeshvi {
    #[doc = " number of vertices"]
    pub vtxCount: ::std::os::raw::c_uint,
    #[doc = " vertices"]
    pub vtx: *mut dReal,
    #[doc = " indices"]
    pub indices: *mut dTriIndex,
    #[doc = " number of indices (all dTriIndex elements)"]
    pub indexCount: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_trimeshvi() {
    const UNINIT: ::std::mem::MaybeUninit<trimeshvi> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<trimeshvi>(),
        32usize,
        concat!("Size of: ", stringify!(trimeshvi))
    );
    assert_eq!(
        ::std::mem::align_of::<trimeshvi>(),
        8usize,
        concat!("Alignment of ", stringify!(trimeshvi))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vtxCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(trimeshvi),
            "::",
            stringify!(vtxCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vtx) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(trimeshvi),
            "::",
            stringify!(vtx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).indices) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(trimeshvi),
            "::",
            stringify!(indices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).indexCount) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(trimeshvi),
            "::",
            stringify!(indexCount)
        )
    );
}
#[doc = " ConvexFVP"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct convexfvp {
    #[doc = " number of planes"]
    pub faceCount: ::std::os::raw::c_uint,
    #[doc = " planes"]
    pub faces: *mut dReal,
    #[doc = " number of vertices"]
    pub vtxCount: ::std::os::raw::c_uint,
    #[doc = " vertices"]
    pub vtx: *mut dReal,
    #[doc = " polygons"]
    pub polygons: *mut ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_convexfvp() {
    const UNINIT: ::std::mem::MaybeUninit<convexfvp> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<convexfvp>(),
        40usize,
        concat!("Size of: ", stringify!(convexfvp))
    );
    assert_eq!(
        ::std::mem::align_of::<convexfvp>(),
        8usize,
        concat!("Alignment of ", stringify!(convexfvp))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).faceCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(convexfvp),
            "::",
            stringify!(faceCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).faces) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(convexfvp),
            "::",
            stringify!(faces)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vtxCount) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(convexfvp),
            "::",
            stringify!(vtxCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vtx) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(convexfvp),
            "::",
            stringify!(vtx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).polygons) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(convexfvp),
            "::",
            stringify!(polygons)
        )
    );
}
extern "C" {
    #[doc = " as local function to be independent of ode.hpp"]
    pub fn _dDot(a: *const dReal, b: *const dReal, n: ::std::os::raw::c_int) -> dReal;
}
extern "C" {
    #[doc = " (defined in ode.hpp)"]
    pub fn dDot(a: *const dReal, b: *const dReal, n: ::std::os::raw::c_int) -> dReal;
}
extern "C" {
    #[doc = " set static in cpp"]
    pub fn SetScaleLimit(sclim: dReal);
}
extern "C" {
    #[doc = " c[3] = a[3] x b[3] same as void dCROSS(c, =, a, b);"]
    pub fn Cross3(c: *mut dReal, a: *mut dReal, b: *mut dReal);
}
extern "C" {
    #[doc = " n[4] = normal(v[9])"]
    pub fn Normal4(n: *mut dReal, v: *mut dReal);
}
extern "C" {
    #[doc = " recalc triangle convexfvp (set normal of faces)"]
    pub fn RecalcFaces(fvp: *mut convexfvp);
}
extern "C" {
    #[doc = " delete vtx, indices when ff is true"]
    pub fn FreeTriMeshVI(tmv: *mut trimeshvi, ff: bool);
}
extern "C" {
    #[doc = " delete faces, vtx, polygons when ff is true"]
    pub fn FreeConvexFVP(fvp: *mut convexfvp, ff: bool);
}
extern "C" {
    #[doc = " always new trimeshvi rescale and return it"]
    pub fn CvtTriMeshVIFromConvexFVP(fvp: *mut convexfvp, sc: dReal) -> *mut trimeshvi;
}
extern "C" {
    #[doc = " always new convexfvp rescale and return it"]
    pub fn CvtConvexFVPFromTriMeshVI(tmv: *mut trimeshvi, sc: dReal) -> *mut convexfvp;
}
extern "C" {
    #[doc = " overwrite trimeshvi rescale and return it"]
    pub fn ScaleTriMeshVI(tmv: *mut trimeshvi, sc: dReal) -> *mut trimeshvi;
}
extern "C" {
    #[doc = " (dst is NULL: new, !NULL: overwrite) trimeshvi rescale and return it"]
    pub fn CopyTriMeshVI(dst: *mut trimeshvi, src: *mut trimeshvi, sc: dReal) -> *mut trimeshvi;
}
extern "C" {
    #[doc = " overwrite convexfvp rescale and return it"]
    pub fn ScaleConvexFVP(fvp: *mut convexfvp, sc: dReal) -> *mut convexfvp;
}
extern "C" {
    #[doc = " (dst is NULL: new, !NULL: overwrite) convexfvp rescale and return it"]
    pub fn CopyConvexFVP(dst: *mut convexfvp, src: *mut convexfvp, sc: dReal) -> *mut convexfvp;
}
