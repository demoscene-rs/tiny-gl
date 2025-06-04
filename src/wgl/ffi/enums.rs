use crate::gl::ffi::GLint;

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt for more information.
// See https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt for more information.

/// True if the pixel format can be used with a window. The
/// <iLayerPlane> parameter is ignored if this attribute is
/// specified.
pub const WGL_DRAW_TO_WINDOW_ARB: GLint = 0x2001;

/// Indicates whether the pixel format is supported by the driver.
/// If this is set to WGL_NO_ACCELERATION_ARB then only the software
/// renderer supports this pixel format; if this is set to
/// WGL_GENERIC_ACCELERATION_ARB then the pixel format is supported
/// by an MCD driver; if this is set to WGL_FULL_ACCELERATION_ARB
/// then the pixel format is supported by an ICD driver.
pub const WGL_ACCELERATION_ARB: GLint = 0x2003;

/// True if OpenGL is supported.
pub const WGL_SUPPORT_OPENGL_ARB: GLint = 0x2010;

/// True if the color buffer has back/front pairs.
pub const WGL_DOUBLE_BUFFER_ARB: GLint = 0x2011;

/// The type of pixel data. This can be set to WGL_TYPE_RGBA_ARB or
/// WGL_TYPE_COLORINDEX_ARB.
pub const WGL_PIXEL_TYPE_ARB: GLint = 0x2013;

/// The number of color bitplanes in each color buffer. For RGBA
/// pixel types, it is the size of the color buffer, excluding the
/// alpha bitplanes. For color-index pixels, it is the size of the
/// color index buffer.
pub const WGL_COLOR_BITS_ARB: GLint = 0x2014;
pub const WGL_ALPHA_BITS_ARB: GLint = 0x201B;
pub const WGL_DEPTH_BITS_ARB: GLint = 0x2022;
pub const WGL_STENCIL_BITS_ARB: GLint = 0x2023;

pub const WGL_FULL_ACCELERATION_ARB: GLint = 0x2027;
pub const WGL_TYPE_RGBA_ARB: GLint = 0x202B;

pub const WGL_SAMPLE_BUFFERS_ARB: GLint = 0x2041;
pub const WGL_SAMPLES_ARB: GLint = 0x2042;

pub const WGL_CONTEXT_MAJOR_VERSION_ARB: GLint = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: GLint = 0x2092;
pub const WGL_CONTEXT_LAYER_PLANE_ARB: GLint = 0x2093;
pub const WGL_CONTEXT_FLAGS_ARB: GLint = 0x2094;

pub const ERROR_INVALID_VERSION_ARB: GLint = 0x2095;
pub const ERROR_INVALID_PROFILE_ARB: GLint = 0x2096;

pub const WGL_CONTEXT_PROFILE_MASK_ARB: GLint = 0x9126;

pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: GLint = 0x0001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: GLint = 0x0002;

pub const WGL_CONTEXT_DEBUG_BIT_ARB: GLint = 0x0001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: GLint = 0x0002;