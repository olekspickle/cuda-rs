#[macro_use]
extern crate enum_primitive;

// Public so downstream crates (e.g. tensorrt-rs) needing raw driver-API
// calls (CUDA graph capture, etc.) go through *this* crate's cuda-rs-sys
// instead of declaring their own separate dependency on it.
pub extern crate cuda_rs_sys as ffi;

#[macro_use]
mod macros;

pub mod context;
pub mod device;
pub mod error;
pub mod event;
pub mod memory;
pub mod stream;

pub fn init() -> Result<(), error::CuError> {
    let res = unsafe { ffi::cuInit(0) };
    wrap!((), res)
}
