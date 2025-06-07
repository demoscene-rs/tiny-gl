#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod gl;
pub(crate) mod types;
pub(crate) mod utils;
pub mod wgl;

use core::ffi::{c_void, CStr};

/// Load OpenGL and WGL functions, using the provided function to get the address of each function.
/// This needs to be called before using any OpenGL or WGL functions.
///
/// # Example
///
/// ```ignore
///     use windows_sys::Win32::Graphics::OpenGL::wglGetProcAddress;
///     use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
///
///     let opengl32 = LoadLibraryA(c"opengl32.dll".as_ptr() as _);
///
///     unsafe {
///         tiny_gl::load(|name| {
///             let addr = wglGetProcAddress(name.as_ptr() as _).map(|ptr| ptr as *const c_void).unwrap_or(core::ptr::null());
///             if addr.is_null() {
///                 GetProcAddress(opengl32, name.as_ptr() as _).unwrap() as _
///             } else {
///                 addr as _
///             }
///         });
///     }
/// ```
pub unsafe fn load(get_proc_address: impl Fn(&CStr) -> *const c_void + Copy) {
    gl::ffi::load(get_proc_address);
    wgl::ffi::load(get_proc_address);
}
