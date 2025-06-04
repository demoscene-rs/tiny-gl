use crate::types::*;

pub const GL_FALSE: GLenum = 0;
pub const GL_TRUE: GLenum = 1;

pub const GL_TEXTURE_1D: GLenum = 0x0DE0;
pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
pub const GL_TEXTURE_3D: GLenum = 0x806F;
pub const GL_TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const GL_TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const GL_TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;

pub const GL_RGBA8: GLenum = 0x8058;
pub const GL_DEPTH24_STENCIL8: GLenum = 0x88F0;

pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_DOUBLE: GLenum = 0x140A;
pub const GL_HALF_FLOAT: GLenum = 0x140B;
pub const GL_FIXED: GLenum = 0x140C;

pub const GL_STREAM_DRAW: GLenum = 0x88E0;
pub const GL_STREAM_READ: GLenum = 0x88E1;
pub const GL_STREAM_COPY: GLenum = 0x88E2;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_STATIC_READ: GLenum = 0x88E5;
pub const GL_STATIC_COPY: GLenum = 0x88E6;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;

pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;

pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;

pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const GL_PATCHES: GLenum = 0x000E;

pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;

pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;

pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const GL_FRAMEBUFFER: GLenum = 0x8D40;