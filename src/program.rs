use gl;
use gl::types::*;

use std::ptr;

use std::rc::Rc;
use std::fmt;
use std::ops::Deref;

use shader::Shader;

pub struct Program {
    id: GLuint,
    shaders: Vec<Rc<Shader>>,
}

impl Eq for Program {}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.id == other.id
    }
}

impl Program {
    pub fn new() -> Result<Program, ProgramError> {
        let program = Program {
            id: unsafe { gl::CreateProgram() },
            shaders: Vec::with_capacity(2),
        };

        if !program.is_program() {
            return Err(ProgramError::CreateFailed);
        }

        Ok(program)
    }

    pub fn link_new(shaders: &[Rc<Shader>]) -> Result<Program, ProgramError> {
        let mut program = match Program::new() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        for shader in shaders {
            if let Err(obj) = program.attach_shader(shader.clone()) {
                return Err(ProgramError::LinkFailed(obj.to_string()));
            }
        }

        if let Err(err) = program.link() {
            return Err(err);
        }

        Ok(program)
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn attach_shader(&mut self, shader: Rc<Shader>) -> Result<(), AttachShaderError> {
        unsafe { gl::AttachShader(self.id, shader.get_id()) };

        match unsafe { gl::GetError() } {
            gl::NO_ERROR => {
                self.shaders.push(shader);
                Ok(())
            },
            _ => Err(AttachShaderError),
        }
    }

    pub fn detach_shader(&mut self, shader: &Shader) -> Result<(), DetachShaderError> {
        unsafe { gl::DetachShader(self.id, shader.get_id()) };

        match unsafe { gl::GetError() } {
            gl::NO_ERROR => {
                self.shaders.retain(|& ref el| el.deref() != shader);
                Ok(())
            },
            _ => Err(DetachShaderError),
        }
    }

    pub fn link(&mut self) -> Result<(), ProgramError> {
        unsafe { gl::LinkProgram(self.id) };

        match self.get_param::<GLint>(gl::LINK_STATUS) {
            Ok(link_status) => {
                if gl::TRUE as GLint == link_status {
                    Ok(())
                } else {
                    match self.get_info_log() {
                        Ok(log) => Err(ProgramError::LinkFailed(log)),
                        Err(err) => Err(err),
                    }
                }
            },
            Err(obj) => Err(ProgramError::Other(obj.to_string())),
        }
    }

    pub fn get_info_log(&self) -> Result<String, ProgramError> {
        match self.get_param::<GLint>(gl::INFO_LOG_LENGTH) {
            Ok(len) => {
                let mut buf = Vec::with_capacity(len as usize);
                unsafe {
                    buf.set_len((len as usize) - 1); // Because null terminated.
                    gl::GetProgramInfoLog(self.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                }

                match String::from_utf8(buf) {
                    Ok(log) => Ok(log),
                    _ => Err(ProgramError::Other("ProgramInfoLog not valid utf8".to_string()))
                }
            },
            Err(obj) => Err(ProgramError::Other(obj.to_string())),
        }
    }

    pub fn get_param<T>(&self, pname: GLenum) -> Result<T, <T as ParamFromProgram>::Err>
        where
            T : ParamFromProgram
    {
        <T as ParamFromProgram>::param_from_program(self, pname)
    }

    pub fn is_program(&self) -> bool {
        unsafe { gl::IsProgram(self.id) == gl::TRUE }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        for shader in self.shaders.drain() {
            unsafe { gl::DetachShader(self.id, shader.get_id()) };
        }

        if self.is_program() {
            unsafe { gl::DeleteProgram(self.id) };
        }
    }
}

pub trait ParamFromProgram {
    /// Output error type.
    type Err;

    /// Gets parameter from the program object.
    fn param_from_program(program: &Program, pname: GLenum) -> Result<Self, Self::Err>;
}

impl ParamFromProgram for GLint {
    type Err = GLintFromProgramError;

    #[inline]
    fn param_from_program(program: &Program, pname: GLenum) -> Result<GLint, GLintFromProgramError> {
        let mut result = gl::FALSE as GLint;
        unsafe { gl::GetProgramiv(program.id, pname, &mut result) };
        match unsafe { gl::GetError() } {
            gl::NO_ERROR => Ok(result),
            error => Err(GLintFromProgramError { info: error }),
        }
    }
}

pub enum ProgramError {
    CreateFailed,
    LinkFailed(String),
    Other(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ProgramError::CreateFailed => "Failed to create program object.".fmt(f),
            &ProgramError::LinkFailed(ref log) => write!(f, "[link failed]\n{}", log),
            &ProgramError::Other(ref err) => write!(f, "[other error]\n{}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttachShaderError;

impl fmt::Display for AttachShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Failed to attach shader to program.".fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DetachShaderError;

impl fmt::Display for DetachShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Failed to detach shader from program.".fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GLintFromProgramError {
    info: GLenum
}

impl fmt::Display for GLintFromProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.info {
            gl::INVALID_ENUM => "Tried to get unnaceptable parameter of program object.".fmt(f),
            gl::INVALID_VALUE => "Tried to get parameter of program which was not generated by OpenGL.".fmt(f),
            gl::INVALID_OPERATION => "Tried to get parameter of program which is not a program object.".fmt(f),
            _ => "Unrecogised error when getting program parameter.".fmt(f)
        }
    }
}
