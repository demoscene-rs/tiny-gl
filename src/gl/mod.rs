use bitflags::bitflags;
use core::{ffi::CStr, num::NonZeroU32};

#[cfg(feature = "alloc")]
use alloc::string::String;

pub mod ffi;

#[derive(Copy, Clone)]
pub struct Shader(pub NonZeroU32);

#[derive(Copy, Clone)]
pub struct Program(pub NonZeroU32);

#[derive(Copy, Clone)]
pub struct Buffer(pub NonZeroU32);

#[derive(Copy, Clone)]
pub struct VertexArray(pub NonZeroU32);

#[derive(Copy, Clone)]
pub struct Texture(pub NonZeroU32);

#[derive(Copy, Clone)]
pub struct Framebuffer(pub NonZeroU32);

#[derive(Copy, Clone)]
pub struct UniformLocation(pub i32);

#[repr(u32)]
pub enum ShaderType {
    Vertex = ffi::GL_VERTEX_SHADER,
    Fragment = ffi::GL_FRAGMENT_SHADER,
    Geometry = ffi::GL_GEOMETRY_SHADER,
    Compute = ffi::GL_COMPUTE_SHADER,
}

#[repr(u32)]
pub enum DrawMode {
    Points = ffi::GL_POINTS,
    Lines = ffi::GL_LINES,
    LineLoop = ffi::GL_LINE_LOOP,
    LineStrip = ffi::GL_LINE_STRIP,
    Triangles = ffi::GL_TRIANGLES,
    TriangleStrip = ffi::GL_TRIANGLE_STRIP,
    TriangleFan = ffi::GL_TRIANGLE_FAN,
    LinesAdjacency = ffi::GL_LINES_ADJACENCY,
    LineStripAdjacency = ffi::GL_LINE_STRIP_ADJACENCY,
    TrianglesAdjacency = ffi::GL_TRIANGLES_ADJACENCY,
    TriangleStripAdjacency = ffi::GL_TRIANGLE_STRIP_ADJACENCY,
    Patches = ffi::GL_PATCHES,
}

#[repr(u32)]
pub enum BufferUsage {
    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by the application, and used as the source for GL drawing and image specification commands.
    StreamDraw = ffi::GL_STREAM_DRAW,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL, and used to return that data when queried by the application.
    StreamRead = ffi::GL_STREAM_READ,

    /// The data store contents will be modified once and used at most a few times.
    /// The data store contents are modified by reading data from the GL, and used as the source for GL drawing and image specification commands.
    StreamCopy = ffi::GL_STREAM_COPY,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by the application, and used as the source for GL drawing and image specification commands.
    StaticDraw = ffi::GL_STATIC_DRAW,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL, and used to return that data when queried by the application.
    StaticRead = ffi::GL_STATIC_READ,

    /// The data store contents will be modified once and used many times.
    /// The data store contents are modified by reading data from the GL, and used as the source for GL drawing and image specification commands.
    StaticCopy = ffi::GL_STATIC_COPY,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by the application, and used as the source for GL drawing and image specification commands.
    DynamicDraw = ffi::GL_DYNAMIC_DRAW,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL, and used to return that data when queried by the application.
    DynamicRead = ffi::GL_DYNAMIC_READ,

    /// The data store contents will be modified repeatedly and used many times.
    /// The data store contents are modified by reading data from the GL, and used as the source for GL drawing and image specification commands.
    DynamicCopy = ffi::GL_DYNAMIC_COPY,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum VertexAttribDataType {
    Byte = ffi::GL_BYTE,
    Short = ffi::GL_SHORT,
    Int = ffi::GL_INT,
    Fixed = ffi::GL_FIXED,
    Float = ffi::GL_FLOAT,
    HalfFloat = ffi::GL_HALF_FLOAT,
    Double = ffi::GL_DOUBLE,
    UnsignedByte = ffi::GL_UNSIGNED_BYTE,
    UnsignedShort = ffi::GL_UNSIGNED_SHORT,
    UnsignedInt = ffi::GL_UNSIGNED_INT,
    // Currently unsupported:
    // GL_INT_2_10_10_10_REV
    // GL_UNSIGNED_INT_2_10_10_10_REV
    // GL_UNSIGNED_INT_10F_11F_11F_REV
}

#[repr(u32)]
pub enum TextureTarget {
    Texture1D = ffi::GL_TEXTURE_1D,
    Texture2D = ffi::GL_TEXTURE_2D,
    Texture3D = ffi::GL_TEXTURE_3D,
    Texture1DArray = ffi::GL_TEXTURE_1D_ARRAY,
    Texture2DArray = ffi::GL_TEXTURE_2D_ARRAY,
    TextureRectangle = ffi::GL_TEXTURE_RECTANGLE,
    TextureCubeMap = ffi::GL_TEXTURE_CUBE_MAP,
    TextureCubeMapArray = ffi::GL_TEXTURE_CUBE_MAP_ARRAY,
    TextureBuffer = ffi::GL_TEXTURE_BUFFER,
    Texture2DMultisample = ffi::GL_TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = ffi::GL_TEXTURE_2D_MULTISAMPLE_ARRAY,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum TextureSizedInternalFormat {
    Rgba8 = ffi::GL_RGBA8,
    Depth24Stencil8 = ffi::GL_DEPTH24_STENCIL8,
    //TODO Others are available...
}

#[derive(Copy, Clone)]
pub enum FramebufferAttachment {
    /// May range from zero to the value of GL_MAX_COLOR_ATTACHMENTS minus one.
    ColorAttachment(u8),
    DepthAttachment,
    StencilAttachment,
    DepthStencilAttachment,
}

impl FramebufferAttachment {
    const fn to_u32(self) -> u32 {
        match self {
            FramebufferAttachment::ColorAttachment(i) => ffi::GL_COLOR_ATTACHMENT0 + i as u32,
            FramebufferAttachment::DepthAttachment => ffi::GL_DEPTH_ATTACHMENT,
            FramebufferAttachment::StencilAttachment => ffi::GL_STENCIL_ATTACHMENT,
            FramebufferAttachment::DepthStencilAttachment => ffi::GL_DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct ClearMask: u32 {
        const COLOR_BUFFER = ffi::GL_COLOR_BUFFER_BIT;
        const DEPTH_BUFFER = ffi::GL_DEPTH_BUFFER_BIT;
        const STENCIL_BUFFER = ffi::GL_STENCIL_BUFFER_BIT;
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Capability {
    DepthTest = ffi::GL_DEPTH_TEST,
    DebugOutput = ffi::GL_DEBUG_OUTPUT,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum DepthFunction {
    Never = ffi::GL_NEVER,
    Less = ffi::GL_LESS,
    Equal = ffi::GL_EQUAL,
    Lequal = ffi::GL_LEQUAL,
    Greater = ffi::GL_GREATER,
    Notequal = ffi::GL_NOTEQUAL,
    Gequal = ffi::GL_GEQUAL,
    Always = ffi::GL_ALWAYS,
}

/// Sets the depth function used for depth buffer comparisons.
/// The initial value is [`DepthFunction::Less`].
///
/// # Notes
/// See [OpenGL documentation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml).
///
/// Wraps [`ffi::glDepthFunc`].
pub unsafe fn depth_function(func: DepthFunction) {
    ffi::glDepthFunc(func as _);
}

/// Enables or disables writing to the depth buffer.
///
/// # Notes
/// See [OpenGL documentation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthMask.xhtml).
///
/// Wraps [`ffi::glDepthMask`].
pub unsafe fn depth_mask(flag: bool) {
    ffi::glDepthMask(flag as _);
}

/// Enables or disables depth testing.
///
/// # Notes
/// See [OpenGL documentation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml).
///
/// Wraps [`ffi::glEnable`] and [`ffi::glDisable`].
pub unsafe fn depth_test(value: bool) {
    if value {
        enable(Capability::DepthTest);
    } else {
        disable(Capability::DepthTest);
    }
}

/// Enables a GL capability.
///
/// # Notes
/// See [OpenGL documentation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml).
///
/// Wraps [`ffi::glEnable`].
pub unsafe fn enable(capability: Capability) {
    ffi::glEnable(capability as _);
}

/// Disables a GL capability.
///
/// # Notes
/// See [OpenGL documentation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml).
///
/// Wraps [`ffi::glDisable`].
pub unsafe fn disable(capability: Capability) {
    ffi::glDisable(capability as _);
}

pub unsafe fn create_shader(ty: ShaderType) -> Option<Shader> {
    NonZeroU32::new(ffi::glCreateShader(ty as _)).map(Shader)
}

pub unsafe fn delete_shader(shader: Shader) {
    ffi::glDeleteShader(shader.0.get());
}

pub unsafe fn shader_source(shader: Shader, source: &str) {
    let ptr = source.as_ptr() as *const ffi::GLchar;
    let length = source.len() as i32;
    ffi::glShaderSource(shader.0.get(), 1, &ptr, &length);
}

pub unsafe fn compile_shader(shader: Shader) {
    ffi::glCompileShader(shader.0.get());
}

pub unsafe fn get_shader_compile_status(shader: Shader) -> bool {
    let mut status = 0;
    ffi::glGetShaderiv(shader.0.get(), ffi::GL_COMPILE_STATUS, &mut status);
    status == 1
}

#[cfg(feature = "alloc")]
pub unsafe fn get_shader_info_log(shader: Shader) -> String {
    let mut length = 0;
    ffi::glGetShaderiv(shader.0.get(), ffi::GL_INFO_LOG_LENGTH, &mut length);

    if length > 0 {
        let mut log = String::with_capacity(length as _);

        ffi::glGetShaderInfoLog(shader.0.get(), length, &mut length, log.as_mut_ptr() as _);

        log.truncate(length as _);
        log
    } else {
        String::new()
    }
}

pub unsafe fn create_program() -> Option<Program> {
    NonZeroU32::new(ffi::glCreateProgram()).map(Program)
}

pub unsafe fn attach_shader(program: Program, shader: Shader) {
    ffi::glAttachShader(program.0.get(), shader.0.get());
}

pub unsafe fn link_program(program: Program) {
    ffi::glLinkProgram(program.0.get());
}

pub unsafe fn use_program(program: Option<Program>) {
    let raw = program.map(|p| p.0.get()).unwrap_or(0);

    ffi::glUseProgram(raw);
}

pub unsafe fn get_program_link_status(program: Program) -> bool {
    let mut status = 0;
    ffi::glGetProgramiv(program.0.get(), ffi::GL_LINK_STATUS, &mut status);
    status == 1
}

pub unsafe fn delete_program(program: Program) {
    ffi::glDeleteProgram(program.0.get());
}

#[cfg(feature = "alloc")]
pub unsafe fn get_program_info_log(program: Program) -> String {
    let mut length = 0;
    ffi::glGetProgramiv(program.0.get(), ffi::GL_INFO_LOG_LENGTH, &mut length);

    if length > 0 {
        let mut log = String::with_capacity(length as _);

        ffi::glGetProgramInfoLog(program.0.get(), length, &mut length, log.as_mut_ptr() as _);

        log.truncate(length as _);
        log
    } else {
        String::new()
    }
}

pub unsafe fn draw_arrays(mode: DrawMode, first: i32, count: i32) {
    ffi::glDrawArrays(mode as _, first, count);
}

pub unsafe fn create_named_buffer() -> Option<Buffer> {
    let mut buffer = 0;
    ffi::glCreateBuffers(1, &mut buffer);
    NonZeroU32::new(buffer).map(Buffer)
}

/// creates and initializes a buffer object's data store.
///
/// # Parameters
/// - `buffer`: The buffer object to initialize.
/// - `size`: The size of the buffer object's new data store.
/// - `usage`: The usage pattern of the buffer object's data store.
///
/// # Notes
/// Wraps [ffi::glNamedBufferData].
pub unsafe fn named_buffer_data_size(buffer: Buffer, size: i32, usage: BufferUsage) {
    ffi::glNamedBufferData(buffer.0.get(), size as _, core::ptr::null(), usage as _);
}

/// creates and initializes a buffer object's data store.
///
/// # Parameters
/// - `buffer`: The buffer object to initialize.
/// - `data`: The data that will be copied into the buffer object's data store for initialization.
/// - `usage`: The usage pattern of the buffer object's data store.
///
/// # Notes
/// Wraps [ffi::glNamedBufferData].
pub unsafe fn named_buffer_data_u8_slice(buffer: Buffer, data: &[u8], usage: BufferUsage) {
    ffi::glNamedBufferData(
        buffer.0.get(),
        data.len() as _,
        data.as_ptr() as _,
        usage as _,
    );
}

pub unsafe fn delete_buffer(buffer: Buffer) {
    ffi::glDeleteBuffers(1, &buffer.0.get());
}

pub unsafe fn create_named_vertex_array() -> Option<VertexArray> {
    let mut vertex_array = 0;
    ffi::glCreateVertexArrays(1, &mut vertex_array);
    NonZeroU32::new(vertex_array).map(VertexArray)
}

pub unsafe fn delete_vertex_array(vertex_array: VertexArray) {
    ffi::glDeleteVertexArrays(1, &vertex_array.0.get());
}

pub unsafe fn enable_vertex_array_attrib(vertex_array: VertexArray, index: u32) {
    ffi::glEnableVertexArrayAttrib(vertex_array.0.get(), index);
}

pub unsafe fn vertex_array_element_buffer(vertex_array: VertexArray, buffer: Buffer) {
    ffi::glVertexArrayElementBuffer(vertex_array.0.get(), buffer.0.get());
}

pub unsafe fn vertex_array_vertex_buffer(
    vertex_array: VertexArray,
    binding_index: u32,
    buffer: Option<Buffer>,
    offset: i32,
    stride: i32,
) {
    let buffer = buffer.map_or(0, |b| b.0.get());
    ffi::glVertexArrayVertexBuffer(
        vertex_array.0.get(),
        binding_index,
        buffer,
        offset as _,
        stride,
    );
}

pub unsafe fn vertex_array_attrib_format(
    vertex_array: VertexArray,
    index: u32,
    size: i32,
    data_type: VertexAttribDataType,
    normalized: bool,
    relative_offset: u32,
) {
    ffi::glVertexArrayAttribFormat(
        vertex_array.0.get(),
        index,
        size,
        data_type as _,
        normalized as _,
        relative_offset as _,
    );
}

pub unsafe fn vertex_array_attrib_binding(
    vertex_array: VertexArray,
    attrib_index: u32,
    binding_index: u32,
) {
    ffi::glVertexArrayAttribBinding(vertex_array.0.get(), attrib_index, binding_index);
}

pub unsafe fn create_texture(target: TextureTarget) -> Option<Texture> {
    let mut texture = 0;
    ffi::glCreateTextures(target as _, 1, &mut texture);
    NonZeroU32::new(texture).map(Texture)
}

pub unsafe fn delete_texture(texture: Texture) {
    ffi::glDeleteTextures(1, &texture.0.get());
}

pub unsafe fn texture_storage_2d(
    texture: Texture,
    levels: i32,
    internal_format: TextureSizedInternalFormat,
    width: i32,
    height: i32,
) {
    ffi::glTextureStorage2D(
        texture.0.get(),
        levels as _,
        internal_format as _,
        width as _,
        height as _,
    );
}

pub unsafe fn create_framebuffer() -> Option<Framebuffer> {
    let mut framebuffer = 0;
    ffi::glCreateFramebuffers(1, &mut framebuffer);
    NonZeroU32::new(framebuffer).map(Framebuffer)
}

pub unsafe fn delete_framebuffer(framebuffer: Framebuffer) {
    ffi::glDeleteFramebuffers(1, &framebuffer.0.get());
}

pub unsafe fn named_framebuffer_texture(
    framebuffer: Framebuffer,
    attachment: FramebufferAttachment,
    texture: Texture,
    level: i32,
) {
    ffi::glNamedFramebufferTexture(
        framebuffer.0.get(),
        attachment.to_u32(),
        texture.0.get(),
        level as _,
    );
}

pub unsafe fn bind_vertex_array(vertex_array: VertexArray) {
    ffi::glBindVertexArray(vertex_array.0.get());
}

//TODO Return Option<UniformLocation> when raw == -1
pub unsafe fn get_uniform_location(program: Program, name: &CStr) -> UniformLocation {
    let raw = ffi::glGetUniformLocation(program.0.get(), name.as_ptr() as _);
    UniformLocation(raw)
}

pub unsafe fn program_uniform_1_f32(program: Program, location: UniformLocation, value: f32) {
    ffi::glProgramUniform1f(program.0.get(), location.0, value);
}

/// Specify the value of a uniform variable for a specified program object
/// This function takes a single 4x4 matrix.
///
/// # Parameters
/// - `program`: The program object to which the uniform variable belongs.
/// - `location`: The location of the uniform variable.
/// - `transpose`: Specifies whether to transpose the matrix.
/// - `value`: The matrix value.
///
/// # Notes
///
/// Wraps [ffi::glProgramUniformMatrix4fv].
pub unsafe fn program_uniform_1_mat4(
    program: Program,
    location: UniformLocation,
    transpose: bool,
    value: &[f32],
) {
    ffi::glProgramUniformMatrix4fv(
        program.0.get(),
        location.0,
        1,
        transpose as _,
        value.as_ptr(),
    );
}

pub unsafe fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    ffi::glClearColor(red, green, blue, alpha);
}

pub unsafe fn clear(mask: ClearMask) {
    ffi::glClear(mask.bits());
}

pub unsafe fn viewport(x: i32, y: i32, width: i32, height: i32) {
    ffi::glViewport(x, y, width, height);
}

//TODO Make functions for all of these:
// glNamedBufferStorage
