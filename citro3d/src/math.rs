//! Safe wrappers for working with matrix and vector types provided by `citro3d`.

// TODO: bench FFI calls into `inline statics` generated by bindgen, vs
// reimplementing some of those calls. Many of them are pretty trivial impls

mod fvec;
mod matrix;
mod ops;
mod projection;

pub use fvec::{FVec, FVec3, FVec4};
pub use matrix::{Matrix, Matrix3, Matrix4};
pub use projection::{
    AspectRatio, ClipPlanes, CoordinateOrientation, Orthographic, Perspective, Projection,
    ScreenOrientation, StereoDisplacement,
};

/// A 4-vector of `u8`s.
#[doc(alias = "C3D_IVec")]
pub struct IVec(citro3d_sys::C3D_IVec);

/// A quaternion, internally represented the same way as [`FVec`].
#[doc(alias = "C3D_FQuat")]
pub struct FQuat(citro3d_sys::C3D_FQuat);
