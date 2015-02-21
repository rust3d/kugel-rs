use gl;
use gl::types::*;

use std::ptr;
use std::ffi::CString;

use std::fmt;

pub struct Shader {
    id: GLuint,
}

impl Eq for Shader {}

impl PartialEq for Shader {
    fn eq(&self, other: &Shader) -> bool {
        self.id == other.id
    }
}

impl Shader {
    #[inline]
    fn internal_new(ty: GLenum) -> Shader {
        Shader {
            id: unsafe { gl::CreateShader(ty) }
        }
    }

    pub fn new(ty: GLenum) -> Shader {
        debug!("new, {}", ty);

        let shader = Shader::internal_new(ty);

        info!("[{}]: created new", shader.id);

        shader
    }

    pub fn compile_new(source: &str, ty: GLenum) -> Result<Shader, ShaderError> {
        debug!("compile new, {}", ty);

        let mut shader = Shader::internal_new(ty);

        shader.set_source(source);

        if let Err(err) = shader.compile() {
            return Err(err);
        }

        info!("[{}]: compiled new", shader.id);

        Ok(shader)
    }

    pub fn compile_vertex_shader(source: &str) -> Result<Shader, ShaderError>
    {
        Shader::compile_new(source, gl::VERTEX_SHADER)
    }

    pub fn compile_fragment_shader(source: &str) -> Result<Shader, ShaderError>
    {
        Shader::compile_new(source, gl::FRAGMENT_SHADER)
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn set_source(&mut self, source: &str) {
        debug!("[{}]: set source", self.id);
        trace!("[{}]: source\n{}", self.id, source);

        let c_str = CString::from_slice(source.as_bytes());
        unsafe { gl::ShaderSource(self.id, 1, &c_str.as_ptr(), ptr::null()); }
    }

    pub fn compile(&mut self) -> Result<(), ShaderError> {
        debug!("[{}]: compile", self.id);

        unsafe { gl::CompileShader(self.id) };

        match self.get_param::<GLint>(gl::COMPILE_STATUS) {
            Ok(compile_status) => {
                if gl::TRUE as GLint == compile_status {
                    debug!("[{}]: compiled", self.id);

                    Ok(())
                } else {
                    match self.get_info_log() {
                        Ok(log) => {
                            error!("[{}]: compile error, {}", self.id, log);

                            Err(ShaderError::CompileFailed(log))
                        },
                        Err(err) => {
                            error!("[{}]: compile error, failed to get info log", self.id);

                            Err(err)
                        },
                    }
                }
            },
            Err(obj) => {
                error!("[{}]: compile error, failed to get status", self.id);

                Err(ShaderError::Other(obj.to_string()))
            },
        }
    }

    pub fn get_info_log(&self) -> Result<String, ShaderError> {
        trace!("[{}]: get info log", self.id);

        match self.get_param::<GLint>(gl::INFO_LOG_LENGTH) {
            Ok(len) => {
                let mut buf = Vec::with_capacity(len as usize);
                unsafe {
                    buf.set_len((len as usize) - 1); // Because null terminated.
                    gl::GetShaderInfoLog(self.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                }

                match String::from_utf8(buf) {
                    Ok(log) => {
                        trace!("[{}]: got info log", self.id);

                        Ok(log)
                    },
                    _ => {
                        let error = ShaderError::Other("ShaderInfoLog not valid utf8".to_string());

                        error!("[{}]: error getting info log, {}", self.id, error);

                        Err(error)
                    }
                }
            },
            Err(obj) => {
                error!("[{}]: error, failed to retrieve log info length", self.id);

                Err(ShaderError::Other(obj.to_string()))
            },
        }
    }

    pub fn get_param<T>(&self, pname: GLenum) -> Result<T, <T as ParamFromShader>::Err>
        where
            T : ParamFromShader
    {
        trace!("[{}]: get param, {}", self.id, pname);
        <T as ParamFromShader>::param_from_shader(self, pname)
    }

    pub fn is_shader(&self) -> bool {
        unsafe { gl::IsShader(self.id) == gl::TRUE }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        debug!("[{}]: cleanup", self.id);

        if self.is_shader() {
            trace!("[{}]: delete", self.id);

            unsafe { gl::DeleteShader(self.id) };
        }
    }
}

pub trait ParamFromShader {
    /// Output error type.
    type Err;

    /// Gets parameter from the shader object.
    fn param_from_shader(shader: &Shader, pname: GLenum) -> Result<Self, Self::Err>;
}

impl ParamFromShader for GLint {
    type Err = GLintFromShaderError;

    #[inline]
    fn param_from_shader(shader: &Shader, pname: GLenum) -> Result<GLint, GLintFromShaderError> {
        let mut result = gl::FALSE as GLint;
        unsafe { gl::GetShaderiv(shader.id, pname, &mut result) };
        match unsafe { gl::GetError() } {
            gl::NO_ERROR => Ok(result),
            error => Err(GLintFromShaderError { info: error }),
        }
    }
}

pub enum ShaderError {
    CompileFailed(String),
    Other(String),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ShaderError::CompileFailed(ref log) => write!(f, "[compile failed]\n{}", log),
            &ShaderError::Other(ref err) => write!(f, "[other error]\n{}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GLintFromShaderError {
    info: GLenum
}

impl fmt::Display for GLintFromShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.info {
            gl::INVALID_ENUM => "Tried to get unnaceptable parameter of shader object.".fmt(f),
            gl::INVALID_VALUE => "Tried to get parameter of shader which was not generated by OpenGL.".fmt(f),
            gl::INVALID_OPERATION => "Tried to get parameter of shader which is not a shader object.".fmt(f),
            _ => "Unrecogised error when getting shader parameter.".fmt(f)
        }
    }
}
