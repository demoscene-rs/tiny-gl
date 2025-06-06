use bitflags::bitflags;
use core::{ffi::c_void, ptr::NonNull};

pub mod ffi;

/// Set the swap interval.
/// See https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/refs/heads/main/extensions/EXT/WGL_EXT_swap_control.txt
///
/// # Notes
/// This wraps [`ffi::wglSwapIntervalEXT`].
pub unsafe fn swap_interval(interval: i32) -> bool {
    ffi::wglSwapIntervalEXT(interval)
}

/// Create a new OpenGL context with the given attributes.
/// See https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/refs/heads/main/extensions/ARB/WGL_ARB_create_context.txt
///
/// # Notes
/// This wraps [`ffi::wglCreateContextAttribsARB`].
/// Make sure that integer_attributes is not empty and ends with IntegerAttribute::End.
pub unsafe fn create_context_attributes(
    hdc: HDC,
    share_context: Option<HGLRC>,
    integer_attributes: &[IntegerAttribute],
) -> Option<HGLRC> {
    #[cfg(debug_assertions)]
    validate_integer_attributes(integer_attributes);

    let share_context = share_context.map_or(core::ptr::null_mut(), NonNull::as_ptr);

    let context = ffi::wglCreateContextAttribsARB(
        hdc.as_ptr(),
        share_context,
        integer_attributes.as_ptr() as _,
    );

    NonNull::new(context)
}

pub type HDC = NonNull<c_void>;
pub type HGLRC = NonNull<c_void>;

/// Represents the acceleration level of the graphics hardware.
/// You almost certainly want to use `Acceleration::Full`.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Acceleration {
    None = ffi::WGL_NO_ACCELERATION_ARB,
    Generic = ffi::WGL_GENERIC_ACCELERATION_ARB,
    Full = ffi::WGL_FULL_ACCELERATION_ARB,
}

/// A boolean value that can be used in integer attributes.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum IntegerBool {
    True = 1,
    False = 0,
}

impl From<bool> for IntegerBool {
    fn from(value: bool) -> Self {
        if value {
            IntegerBool::True
        } else {
            IntegerBool::False
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PixelType {
    Rgba = ffi::WGL_TYPE_RGBA_ARB,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ProfileMask: i32 {
        const Core = ffi::WGL_CONTEXT_CORE_PROFILE_BIT_ARB;
        const Compatibility = ffi::WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct ContextFlags: i32 {
        const Debug = ffi::WGL_CONTEXT_DEBUG_BIT_ARB;
        const ForwardCompatible = ffi::WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB;
    }
}

/// Various attributes that can be used in creating a context or choosing a pixel format.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C, i32)]
pub enum IntegerAttribute {
    End = 0,

    DrawToWindow(IntegerBool) = ffi::WGL_DRAW_TO_WINDOW_ARB,
    SupportOpenGL(IntegerBool) = ffi::WGL_SUPPORT_OPENGL_ARB,
    DoubleBuffer(IntegerBool) = ffi::WGL_DOUBLE_BUFFER_ARB,
    Acceleration(Acceleration) = ffi::WGL_ACCELERATION_ARB,
    PixelType(PixelType) = ffi::WGL_PIXEL_TYPE_ARB,
    ColorBits(i32) = ffi::WGL_COLOR_BITS_ARB,
    DepthBits(i32) = ffi::WGL_DEPTH_BITS_ARB,
    AlphaBits(i32) = ffi::WGL_ALPHA_BITS_ARB,
    StencilBits(i32) = ffi::WGL_STENCIL_BITS_ARB,
    SampleBuffers(IntegerBool) = ffi::WGL_SAMPLE_BUFFERS_ARB,
    Samples(i32) = ffi::WGL_SAMPLES_ARB,

    //TODO Split these off into their own enum
    ContextMajorVersion(i32) = ffi::WGL_CONTEXT_MAJOR_VERSION_ARB,
    ContextMinorVersion(i32) = ffi::WGL_CONTEXT_MINOR_VERSION_ARB,
    ContextProfileMask(ProfileMask) = ffi::WGL_CONTEXT_PROFILE_MASK_ARB,
    ContextFlags(ContextFlags) = ffi::WGL_CONTEXT_FLAGS_ARB,
}

fn validate_integer_attributes(attributes: &[IntegerAttribute]) {
    if attributes.is_empty() {
        panic!("IntegerAttributes list is empty")
    }

    let last = unsafe { *attributes.last().unwrap_unchecked() };

    if last != IntegerAttribute::End {
        panic!("IntegerAttributes list does not end with IntegerAttribute::End")
    }
}

/// Chooses a pixel format with the given attributes.
/// See https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/refs/heads/main/extensions/ARB/WGL_ARB_pixel_format.txt
///
/// # Notes
/// This wraps [`ffi::wglChoosePixelFormatARB`].
/// Make sure that integer_attributes is not empty and ends with IntegerAttribute::End.
pub unsafe fn choose_pixel_format(
    hdc: HDC,
    integer_attributes: &[IntegerAttribute],
    max_formats: u32,
    pixel_format: &mut i32,
    num_formats: &mut u32,
) -> bool {
    #[cfg(debug_assertions)]
    validate_integer_attributes(integer_attributes);

    ffi::wglChoosePixelFormatARB(
        hdc.as_ptr(),
        integer_attributes.as_ptr() as _,
        core::ptr::null(),
        max_formats,
        pixel_format,
        num_formats,
    )
}
