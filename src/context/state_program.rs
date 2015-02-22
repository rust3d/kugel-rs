use gl;

use std::rc::Rc;

use program::Program;

pub struct StateProgram {
    program: Option<Rc<Program>>,
}

impl StateProgram {
    pub fn new() -> StateProgram {
        StateProgram {
            program: None,
        }
    }

    pub fn using(&mut self, program: &Rc<Program>) -> Result<&mut StateProgram, UseProgramError> {
        debug!("[{}]: use", program.get_id());

        self.program = Some(program.clone());
        unsafe { gl::UseProgram(program.get_id()) };
        Ok(self)
    }

    pub fn unuse(&mut self) {
        if let Some(ref program) = self.program {
            debug!("[{}]: unuse", program.get_id());

            unsafe { gl::UseProgram(0) };
        }

        self.program = None;
    }
}

impl Drop for StateProgram {
    fn drop(&mut self) {
        self.unuse();
    }
}

pub struct UseProgramError;
