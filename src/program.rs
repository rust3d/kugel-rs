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
    #[inline]
    fn internal_new() -> Program {
        Program {
            id: unsafe { gl::CreateProgram() },
            shaders: Vec::with_capacity(2),
        }
    }

    pub fn new() -> Program {
        debug!("new");

        let program = Program::internal_new();

        info!("[{}]: created new", program.id);

        program
    }

    pub fn link_new(shaders: &[Rc<Shader>]) -> Result<Program, ProgramError> {
        debug!("link new");

        let mut program = Program::internal_new();

        for shader in shaders {
            if let Err(obj) = program.attach_shader(shader.clone()) {
                return Err(ProgramError::LinkFailed(obj.to_string()));
            }
        }

        if let Err(err) = program.link() {
            return Err(err);
        }

        info!("[{}]: linked new", program.id, );

        Ok(program)
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn attach_shader(&mut self, shader: Rc<Shader>) -> Result<(), AttachShaderError> {
        debug!("[{}]: attach shader, {}", self.id, shader.get_id());

        unsafe { gl::AttachShader(self.id, shader.get_id()) };

        match unsafe { gl::GetError() } {
            gl::NO_ERROR => {
                trace!("[{}]: attached shader {}", self.id, shader.get_id());

                self.shaders.push(shader);
                Ok(())
            },
            _ => {
                error!("[{}]: attaching shader {}, {}", self.id, shader.get_id(), AttachShaderError);

                Err(AttachShaderError)
            },
        }
    }

    pub fn detach_shader(&mut self, shader: &Shader) -> Result<(), DetachShaderError> {
        debug!("[{}]: detach shader, {}", self.id, shader.get_id());

        unsafe { gl::DetachShader(self.id, shader.get_id()) };

        match unsafe { gl::GetError() } {
            gl::NO_ERROR => {
                trace!("[{}]: detached shader, {}", self.id, shader.get_id());

                self.shaders.retain(|& ref el| el.deref() != shader);
                Ok(())
            },
            _ => {
                error!("[{}]: detach shader {}, {}", self.id, shader.get_id(), DetachShaderError);

                Err(DetachShaderError)
            },
        }
    }

    pub fn link(&mut self) -> Result<(), ProgramError> {
        debug!("[{}]: link", self.id);

        unsafe { gl::LinkProgram(self.id) };

        match self.get_param::<GLint>(gl::LINK_STATUS) {
            Ok(link_status) => {
                if gl::TRUE as GLint == link_status {
                    trace!("[{}]: linked", self.id);

                    Ok(())
                } else {
                    match self.get_info_log() {
                        Ok(log) => {
                            error!("[{}]: link error, {}", self.id, log);

                            Err(ProgramError::LinkFailed(log))
                        },
                        Err(err) => {
                            error!("[{}]: link error, failed to retrieve log", self.id);

                            Err(err)
                        },
                    }
                }
            },
            Err(obj) => {
                error!("[{}]: link error, failed to retrieve status", self.id);

                Err(ProgramError::Other(obj.to_string()))
            },
        }
    }

    pub fn get_info_log(&self) -> Result<String, ProgramError> {
        trace!("[{}]: get info log", self.id);

        match self.get_param::<GLint>(gl::INFO_LOG_LENGTH) {
            Ok(len) => {
                let mut buf = Vec::with_capacity(len as usize);
                unsafe {
                    buf.set_len((len as usize) - 1); // Because null terminated.
                    gl::GetProgramInfoLog(self.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                }

                match String::from_utf8(buf) {
                    Ok(log) => {
                        trace!("[{}]: got info log", self.id);

                        Ok(log)
                    },
                    _ => {
                        let error = ProgramError::Other("ProgramInfoLog not valid utf8".to_string());

                        error!("[{}]: error getting info log, {}", self.id, error);

                        Err(error)
                    }
                }
            },
            Err(obj) => {
                error!("[{}]: error, failed to retrieve log info length", self.id);

                Err(ProgramError::Other(obj.to_string()))
            },
        }
    }

    pub fn get_param<T>(&self, pname: GLenum) -> Result<T, <T as ParamFromProgram>::Err>
        where
            T : ParamFromProgram
    {
        trace!("[{}]: get param, {}", self.id, pname);
        <T as ParamFromProgram>::param_from_program(self, pname)
    }

    pub fn is_program(&self) -> bool {
        unsafe { gl::IsProgram(self.id) == gl::TRUE }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        debug!("[{}]: cleanup", self.id);

        for shader in self.shaders.drain() {
            unsafe { gl::DetachShader(self.id, shader.get_id()) };
        }

        if self.is_program() {
            trace!("[{}]: delete", self.id);

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
    LinkFailed(String),
    Other(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
