#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod gl;
pub(crate) mod types;
pub(crate) mod utils;
pub mod wgl;

use core::ffi::{c_void, CStr};

pub unsafe fn load(get_proc_address: impl Fn(&CStr) -> *const c_void + Copy) {
    gl::ffi::load(get_proc_address);
    wgl::ffi::load(get_proc_address);
}
