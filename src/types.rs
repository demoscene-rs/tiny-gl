use core::ffi::{c_char, c_float, c_int, c_uchar, c_uint};

pub type GLenum = c_uint;
pub type GLuint = c_uint;
pub type GLchar = c_char;
pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLsizeiptr = isize;
pub type GLintptr = isize;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLfloat = c_float;