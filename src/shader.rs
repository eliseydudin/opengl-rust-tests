use core::{error, fmt, marker::PhantomData, ptr};
use std::ffi::{CStr, CString, NulError, c_char};

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Fragment = gl::FRAGMENT_SHADER,
    Vertex = gl::VERTEX_SHADER,
}

pub trait AsShaderType {
    fn as_shader_type() -> ShaderType;
}

pub struct Fragment;

impl AsShaderType for Fragment {
    fn as_shader_type() -> ShaderType {
        ShaderType::Fragment
    }
}

pub struct Vertex;

impl AsShaderType for Vertex {
    fn as_shader_type() -> ShaderType {
        ShaderType::Vertex
    }
}

pub struct Shader<S: AsShaderType> {
    pub(crate) handle: u32,
    data: PhantomData<S>,
}

#[derive(Debug)]
pub enum ShaderError {
    CompilationError(String),
    LinkingError(String),
    CStringConversion(NulError),
    UnknownUniformLocation(String),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompilationError(err) => write!(f, "Cannot compile the shader: {err}"),
            Self::CStringConversion(err) => {
                write!(f, "Cannot convert string to a C pointer: {err}")
            }
            Self::LinkingError(err) => write!(f, "Cannot link the shaders: {err}"),
            Self::UnknownUniformLocation(s) => write!(f, "Unknown uniform location: {s}"),
        }
    }
}

impl error::Error for ShaderError {}

impl<S: AsShaderType> Shader<S> {
    pub fn new(source: &str) -> Result<Self, ShaderError> {
        let source = CString::new(source).map_err(ShaderError::CStringConversion)?;
        Self::from_cstr(&source)
    }

    pub fn from_cstr(cstr: &CStr) -> Result<Self, ShaderError> {
        unsafe {
            let ptr = cstr.as_ptr();
            let id = gl::CreateShader(S::as_shader_type() as u32);
            gl::ShaderSource(id, 1, ptr::addr_of!(ptr), ptr::null());
            gl::CompileShader(id);

            if !Self::check_compile_status(id) {
                return Err(Self::get_error(id));
            }

            Ok(Self {
                handle: id,
                data: PhantomData,
            })
        }
    }

    fn get_error(id: u32) -> ShaderError {
        let mut buffer = [c_char::MIN; 512];
        unsafe { gl::GetShaderInfoLog(id, 512, ptr::null_mut(), buffer.as_mut_ptr()) };

        let err = unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .to_string();

        ShaderError::CompilationError(err)
    }

    fn check_compile_status(id: u32) -> bool {
        let mut success = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, ptr::addr_of_mut!(success)) };

        success != 0
    }
}

impl<S: AsShaderType> Drop for Shader<S> {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle) }
    }
}

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new(
        vertex_shader: Shader<Vertex>,
        fragment_shader: Shader<Fragment>,
    ) -> Result<Self, ShaderError> {
        Self::link_internal(vertex_shader, fragment_shader).map(|id| Self { id })
    }

    fn link_internal(
        vertex_shader: Shader<Vertex>,
        fragment_shader: Shader<Fragment>,
    ) -> Result<u32, ShaderError> {
        let id = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader.handle);
            gl::AttachShader(id, fragment_shader.handle);
            gl::LinkProgram(id);
            id
        };

        if !Self::check_link_status(id) {
            return Err(Self::get_error(id));
        }

        Ok(id)
    }

    fn get_error(id: u32) -> ShaderError {
        let mut buffer = [c_char::MIN; 512];
        unsafe { gl::GetProgramInfoLog(id, 512, ptr::null_mut(), buffer.as_mut_ptr()) };

        let err = unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .to_string();

        ShaderError::LinkingError(err)
    }

    fn check_link_status(id: u32) -> bool {
        let mut success = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, ptr::addr_of_mut!(success)) };

        success != 0
    }
}

impl Program {
    pub fn use_internal(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn put_uniform<U>(&self, position: &str, uniform: &U) -> Result<(), ShaderError>
    where
        U: Uniform,
    {
        let uniform_position = match CString::new(position) {
            Ok(pos) => pos,
            Err(e) => return Err(ShaderError::CStringConversion(e)),
        };

        let uniform_position =
            unsafe { gl::GetUniformLocation(self.id, uniform_position.as_ptr()) };
        if uniform_position == -1 {
            return Err(ShaderError::UnknownUniformLocation(position.to_owned()));
        }

        self.use_internal();
        unsafe { uniform.put_uniform(uniform_position) };

        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum AttributeType {
    i8 = gl::BYTE,
    u8 = gl::UNSIGNED_BYTE,
    i16 = gl::SHORT,
    u16 = gl::UNSIGNED_SHORT,
    f32 = gl::FLOAT,
    f16 = gl::HALF_FLOAT,
    f64 = gl::DOUBLE,
    i32 = gl::INT,
    u32 = gl::UNSIGNED_INT,
    fixed = gl::FIXED,
}

pub fn setup_attribute(
    index: u32,
    size: i32,
    offset: u32,
    stride: i32,
    attribute_type: AttributeType,
) {
    unsafe {
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(
            index,
            size,
            attribute_type as u32,
            gl::FALSE,
            stride * std::mem::size_of::<f32>() as i32,
            (offset * std::mem::size_of::<f32>() as u32) as *const _,
        );
    }
}

pub trait Uniform {
    unsafe fn put_uniform(&self, pos: i32);
}

impl Uniform for f32 {
    unsafe fn put_uniform(&self, pos: i32) {
        unsafe { gl::Uniform1f(pos, *self) }
    }
}

impl Uniform for nalgebra_glm::Mat4 {
    unsafe fn put_uniform(&self, pos: i32) {
        unsafe { gl::UniformMatrix4fv(pos, 1, gl::FALSE, self.as_ptr()) }
    }
}
