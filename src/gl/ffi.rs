#![allow(non_snake_case)]

mod enums;

use crate::utils::*;
use core::ffi::{c_void, CStr};

pub use crate::types::*;
pub use enums::*;

pub unsafe fn glCreateShader(ty: GLenum) -> GLuint {
    call_ptr_1arg(CREATE_SHADER_PTR, ty)
}

pub unsafe fn glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    call_ptr_4arg(SHADER_SOURCE_PTR, shader, count, string, length)
}

pub unsafe fn glCompileShader(shader: GLuint) {
    call_ptr_1arg(COMPILE_SHADER_PTR, shader)
}

pub unsafe fn glDeleteShader(shader: GLuint) {
    call_ptr_1arg(DELETE_SHADER_PTR, shader)
}

pub unsafe fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    call_ptr_3arg(GET_SHADER_IV_PTR, shader, pname, params)
}

pub unsafe fn glGetShaderInfoLog(
    shader: GLuint,
    max_length: GLsizei,
    length: *mut GLsizei,
    info_log: *mut GLchar,
) {
    call_ptr_4arg(
        GET_SHADER_INFO_LOG_PTR,
        shader,
        max_length,
        length,
        info_log,
    )
}

pub unsafe fn glCreateProgram() -> GLuint {
    call_ptr_0arg(CREATE_PROGRAM_PTR)
}

pub unsafe fn glAttachShader(program: GLuint, shader: GLuint) {
    call_ptr_2arg(ATTACH_SHADER_PTR, program, shader)
}

pub unsafe fn glLinkProgram(program: GLuint) {
    call_ptr_1arg(LINK_PROGRAM_PTR, program)
}

pub unsafe fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    call_ptr_3arg(GET_PROGRAM_IV_PTR, program, pname, params)
}

pub unsafe fn glGetProgramInfoLog(
    program: GLuint,
    max_length: GLsizei,
    length: *mut GLsizei,
    info_log: *mut GLchar,
) {
    call_ptr_4arg(
        GET_PROGRAM_INFO_LOG_PTR,
        program,
        max_length,
        length,
        info_log,
    )
}

pub unsafe fn glDeleteProgram(program: GLuint) {
    call_ptr_1arg(DELETE_PROGRAM_PTR, program)
}

pub unsafe fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    call_ptr_3arg(DRAW_ARRAYS_PTR, mode, first, count)
}

pub unsafe fn glCreateBuffers(n: GLsizei, buffers: *mut GLuint) {
    call_ptr_2arg(CREATE_BUFFERS_PTR, n, buffers)
}

pub unsafe fn glNamedBufferData(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum,
) {
    call_ptr_4arg(NAMED_BUFFER_DATA_PTR, buffer, size, data, usage)
}

pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    call_ptr_2arg(DELETE_BUFFERS_PTR, n, buffers)
}

pub unsafe fn glCreateVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    call_ptr_2arg(CREATE_VERTEX_ARRAYS_PTR, n, arrays)
}

pub unsafe fn glVertexArrayVertexBuffer(
    vaobj: GLuint,
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    call_ptr_5arg(
        VERTEX_ARRAY_VERTEX_BUFFER_PTR,
        vaobj,
        bindingindex,
        buffer,
        offset,
        stride,
    )
}

pub unsafe fn glVertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) {
    call_ptr_2arg(VERTEX_ARRAY_ELEMENT_BUFFER_PTR, vaobj, buffer)
}

pub unsafe fn glEnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    call_ptr_2arg(ENABLE_VERTEX_ARRAY_ATTRIB_PTR, vaobj, index)
}

pub unsafe fn glVertexArrayAttribFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    ty: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    call_ptr_6arg(
        VERTEX_ARRAY_ATTRIB_FORMAT_PTR,
        vaobj,
        attribindex,
        size,
        ty,
        normalized,
        relativeoffset,
    )
}

pub unsafe fn glVertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) {
    call_ptr_3arg(
        VERTEX_ARRAY_ATTRIB_BINDING_PTR,
        vaobj,
        attribindex,
        bindingindex,
    )
}

pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    call_ptr_2arg(DELETE_VERTEX_ARRAYS_PTR, n, arrays)
}

pub unsafe fn glNamedBufferStorage(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const c_void,
    flags: GLbitfield,
) {
    call_ptr_4arg(NAMED_BUFFER_STORAGE_PTR, buffer, size, data, flags)
}

pub unsafe fn glCreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) {
    call_ptr_3arg(CREATE_TEXTURES_PTR, target, n, textures)
}

pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    call_ptr_2arg(DELETE_TEXTURES_PTR, n, textures)
}

pub unsafe fn glTextureStorage2D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    call_ptr_5arg(
        TEXTURE_STORAGE_2D_PTR,
        texture,
        levels,
        internalformat,
        width,
        height,
    )
}

pub unsafe fn glCreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    call_ptr_2arg(CREATE_FRAMEBUFFERS_PTR, n, framebuffers)
}

pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    call_ptr_2arg(DELETE_FRAMEBUFFERS_PTR, n, framebuffers)
}

pub unsafe fn glNamedFramebufferTexture(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    call_ptr_4arg(
        NAMED_FRAMEBUFFER_TEXTURE_PTR,
        framebuffer,
        attachment,
        texture,
        level,
    )
}

pub unsafe fn glUseProgram(program: GLuint) {
    call_ptr_1arg(USE_PROGRAM_PTR, program)
}

pub unsafe fn glBindVertexArray(vertex_array: GLuint) {
    call_ptr_1arg(BIND_VERTEX_ARRAY_PTR, vertex_array)
}

pub unsafe fn glClear(mask: GLbitfield) {
    call_ptr_1arg(CLEAR_PTR, mask)
}

pub unsafe fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    call_ptr_4arg(CLEAR_COLOR_PTR, red, green, blue, alpha)
}

pub unsafe fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    call_ptr_2arg(GET_UNIFORM_LOCATION_PTR, program, name)
}

pub unsafe fn glProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) {
    call_ptr_3arg(PROGRAM_UNIFORM_1F_PTR, program, location, v0)
}

pub unsafe fn glProgramUniformMatrix4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    call_ptr_5arg(
        PROGRAM_UNIFORM_MATRIX_5FV_PTR,
        program,
        location,
        count,
        transpose,
        value,
    )
}

pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    call_ptr_4arg(VIEWPORT_PTR, x, y, width, height)
}

pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    call_ptr_2arg(BIND_FRAMEBUFFER_PTR, target, framebuffer)
}

pub unsafe fn glEnable(cap: GLenum) {
    call_ptr_1arg(ENABLE_PTR, cap)
}

pub unsafe fn glDisable(cap: GLenum) {
    call_ptr_1arg(DISABLE_PTR, cap)
}

pub unsafe fn glDepthMask(flag: GLboolean) {
    call_ptr_1arg(DEPTH_MASK_PTR, flag)
}

pub unsafe fn glDepthFunc(func: GLenum) {
    call_ptr_1arg(DEPTH_FUNC_PTR, func)
}

static mut CREATE_SHADER_PTR: *const c_void = core::ptr::null();
static mut SHADER_SOURCE_PTR: *const c_void = core::ptr::null();
static mut COMPILE_SHADER_PTR: *const c_void = core::ptr::null();
static mut DELETE_SHADER_PTR: *const c_void = core::ptr::null();
static mut GET_SHADER_IV_PTR: *const c_void = core::ptr::null();
static mut GET_SHADER_INFO_LOG_PTR: *const c_void = core::ptr::null();
static mut CREATE_PROGRAM_PTR: *const c_void = core::ptr::null();
static mut ATTACH_SHADER_PTR: *const c_void = core::ptr::null();
static mut LINK_PROGRAM_PTR: *const c_void = core::ptr::null();
static mut GET_PROGRAM_IV_PTR: *const c_void = core::ptr::null();
static mut GET_PROGRAM_INFO_LOG_PTR: *const c_void = core::ptr::null();
static mut DELETE_PROGRAM_PTR: *const c_void = core::ptr::null();
static mut DRAW_ARRAYS_PTR: *const c_void = core::ptr::null();
static mut CREATE_BUFFERS_PTR: *const c_void = core::ptr::null();
static mut NAMED_BUFFER_DATA_PTR: *const c_void = core::ptr::null();
static mut DELETE_BUFFERS_PTR: *const c_void = core::ptr::null();
static mut CREATE_VERTEX_ARRAYS_PTR: *const c_void = core::ptr::null();
static mut VERTEX_ARRAY_VERTEX_BUFFER_PTR: *const c_void = core::ptr::null();
static mut VERTEX_ARRAY_ELEMENT_BUFFER_PTR: *const c_void = core::ptr::null();
static mut ENABLE_VERTEX_ARRAY_ATTRIB_PTR: *const c_void = core::ptr::null();
static mut VERTEX_ARRAY_ATTRIB_FORMAT_PTR: *const c_void = core::ptr::null();
static mut VERTEX_ARRAY_ATTRIB_BINDING_PTR: *const c_void = core::ptr::null();
static mut DELETE_VERTEX_ARRAYS_PTR: *const c_void = core::ptr::null();
static mut NAMED_BUFFER_STORAGE_PTR: *const c_void = core::ptr::null();
static mut CREATE_TEXTURES_PTR: *const c_void = core::ptr::null();
static mut DELETE_TEXTURES_PTR: *const c_void = core::ptr::null();
static mut TEXTURE_STORAGE_2D_PTR: *const c_void = core::ptr::null();
static mut CREATE_FRAMEBUFFERS_PTR: *const c_void = core::ptr::null();
static mut DELETE_FRAMEBUFFERS_PTR: *const c_void = core::ptr::null();
static mut NAMED_FRAMEBUFFER_TEXTURE_PTR: *const c_void = core::ptr::null();
static mut USE_PROGRAM_PTR: *const c_void = core::ptr::null();
static mut BIND_VERTEX_ARRAY_PTR: *const c_void = core::ptr::null();
static mut CLEAR_PTR: *const c_void = core::ptr::null();
static mut CLEAR_COLOR_PTR: *const c_void = core::ptr::null();
static mut GET_UNIFORM_LOCATION_PTR: *const c_void = core::ptr::null();
static mut PROGRAM_UNIFORM_1F_PTR: *const c_void = core::ptr::null();
static mut PROGRAM_UNIFORM_MATRIX_5FV_PTR: *const c_void = core::ptr::null();
static mut VIEWPORT_PTR: *const c_void = core::ptr::null();
static mut BIND_FRAMEBUFFER_PTR: *const c_void = core::ptr::null();
static mut ENABLE_PTR: *const c_void = core::ptr::null();
static mut DISABLE_PTR: *const c_void = core::ptr::null();
static mut DEPTH_MASK_PTR: *const c_void = core::ptr::null();
static mut DEPTH_FUNC_PTR: *const c_void = core::ptr::null();

//TODO Implement for textures:
// glTextureParameteri
// glTextureSubImage2D
// glBindTextureUnit
// glGenerateTextureMipmap

//TODO Implement for framebuffers:
// glCheckNamedFramebufferStatus
// glBlitNamedFramebuffer
// glClearNamedFramebufferfv

//TODO Others:
// glGetError

pub unsafe fn load(get_proc_address: impl Fn(&CStr) -> *const c_void) {
    CREATE_SHADER_PTR = get_proc_address(c"glCreateShader");
    SHADER_SOURCE_PTR = get_proc_address(c"glShaderSource");
    COMPILE_SHADER_PTR = get_proc_address(c"glCompileShader");
    DELETE_SHADER_PTR = get_proc_address(c"glDeleteShader");
    GET_SHADER_IV_PTR = get_proc_address(c"glGetShaderiv");
    GET_SHADER_INFO_LOG_PTR = get_proc_address(c"glGetShaderInfoLog");
    CREATE_PROGRAM_PTR = get_proc_address(c"glCreateProgram");
    ATTACH_SHADER_PTR = get_proc_address(c"glAttachShader");
    LINK_PROGRAM_PTR = get_proc_address(c"glLinkProgram");
    GET_PROGRAM_IV_PTR = get_proc_address(c"glGetProgramiv");
    GET_PROGRAM_INFO_LOG_PTR = get_proc_address(c"glGetProgramInfoLog");
    DELETE_PROGRAM_PTR = get_proc_address(c"glDeleteProgram");
    DRAW_ARRAYS_PTR = get_proc_address(c"glDrawArrays");
    CREATE_BUFFERS_PTR = get_proc_address(c"glCreateBuffers");
    NAMED_BUFFER_DATA_PTR = get_proc_address(c"glNamedBufferData");
    DELETE_BUFFERS_PTR = get_proc_address(c"glDeleteBuffers");
    CREATE_VERTEX_ARRAYS_PTR = get_proc_address(c"glCreateVertexArrays");
    VERTEX_ARRAY_VERTEX_BUFFER_PTR = get_proc_address(c"glVertexArrayVertexBuffer");
    VERTEX_ARRAY_ELEMENT_BUFFER_PTR = get_proc_address(c"glVertexArrayElementBuffer");
    ENABLE_VERTEX_ARRAY_ATTRIB_PTR = get_proc_address(c"glEnableVertexArrayAttrib");
    VERTEX_ARRAY_ATTRIB_FORMAT_PTR = get_proc_address(c"glVertexArrayAttribFormat");
    VERTEX_ARRAY_ATTRIB_BINDING_PTR = get_proc_address(c"glVertexArrayAttribBinding");
    DELETE_VERTEX_ARRAYS_PTR = get_proc_address(c"glDeleteVertexArrays");
    NAMED_BUFFER_STORAGE_PTR = get_proc_address(c"glNamedBufferStorage");
    CREATE_TEXTURES_PTR = get_proc_address(c"glCreateTextures");
    DELETE_TEXTURES_PTR = get_proc_address(c"glDeleteTextures");
    TEXTURE_STORAGE_2D_PTR = get_proc_address(c"glTextureStorage2D");
    CREATE_FRAMEBUFFERS_PTR = get_proc_address(c"glCreateFramebuffers");
    DELETE_FRAMEBUFFERS_PTR = get_proc_address(c"glDeleteFramebuffers");
    NAMED_FRAMEBUFFER_TEXTURE_PTR = get_proc_address(c"glNamedFramebufferTexture");
    USE_PROGRAM_PTR = get_proc_address(c"glUseProgram");
    BIND_VERTEX_ARRAY_PTR = get_proc_address(c"glBindVertexArray");
    CLEAR_PTR = get_proc_address(c"glClear");
    CLEAR_COLOR_PTR = get_proc_address(c"glClearColor");
    GET_UNIFORM_LOCATION_PTR = get_proc_address(c"glGetUniformLocation");
    PROGRAM_UNIFORM_1F_PTR = get_proc_address(c"glProgramUniform1f");
    PROGRAM_UNIFORM_MATRIX_5FV_PTR = get_proc_address(c"glProgramUniformMatrix4fv");
    VIEWPORT_PTR = get_proc_address(c"glViewport");
    BIND_FRAMEBUFFER_PTR = get_proc_address(c"glBindFramebuffer");
    ENABLE_PTR = get_proc_address(c"glEnable");
    DISABLE_PTR = get_proc_address(c"glDisable");
    DEPTH_MASK_PTR = get_proc_address(c"glDepthMask");
    DEPTH_FUNC_PTR = get_proc_address(c"glDepthFunc");
}
