#![allow(non_snake_case)]

use core::ffi::{c_void, CStr};
use crate::utils::*;

pub use crate::types::*;
pub use enums::*;

mod enums;

static mut CHOOSE_PIXEL_FORMAT_ARB_PTR: *const c_void = core::ptr::null();
static mut CREATE_CONTEXT_ATTRIBS_ARB_PTR: *const c_void = core::ptr::null();
static mut SWAP_INTERVAL_EXT_PTR: *const c_void = core::ptr::null();


pub unsafe fn wglChoosePixelFormatARB(hdc: *mut c_void, piAttribIList: *const GLint, pfAttribFList: *const f32, nMaxFormats: GLuint, piFormats: *mut GLint, nNumFormats: *mut GLuint) -> bool {
    call_ptr_6arg(CHOOSE_PIXEL_FORMAT_ARB_PTR, hdc, piAttribIList, pfAttribFList, nMaxFormats, piFormats, nNumFormats)
}

pub unsafe fn wglCreateContextAttribsARB(hdc: *mut c_void, hShareContext: *mut c_void, attribList: *const GLint) -> *mut c_void {
    call_ptr_3arg(CREATE_CONTEXT_ATTRIBS_ARB_PTR, hdc, hShareContext, attribList)
}

// See https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/refs/heads/main/extensions/EXT/WGL_EXT_swap_control.txt
pub unsafe fn wglSwapIntervalEXT(interval: GLint) -> bool {
    call_ptr_1arg(SWAP_INTERVAL_EXT_PTR, interval)
}

pub unsafe fn load(get_proc_address: impl Fn(&CStr)-> *const c_void) {
    CHOOSE_PIXEL_FORMAT_ARB_PTR = get_proc_address(c"wglChoosePixelFormatARB");
    CREATE_CONTEXT_ATTRIBS_ARB_PTR = get_proc_address(c"wglCreateContextAttribsARB");
    SWAP_INTERVAL_EXT_PTR = get_proc_address(c"wglSwapIntervalEXT");
}