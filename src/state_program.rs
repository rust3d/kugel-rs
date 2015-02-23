use gl;
use gl::types::*;

use program::Program;

pub struct StateProgram {
    id: Option<GLuint>,
}

impl StateProgram {
    pub fn new() -> StateProgram {
        StateProgram {
            id: None,
        }
    }

    pub fn with_use(&mut self, program: &mut Program) -> &mut StateProgram {
        let previous_id = self.id;
        let new_id = program.get_id();

        match previous_id {
            Some(used_id) => {
                if used_id != new_id {
                    debug!("[{}]: replace previous {}", new_id, used_id);

                    self.id = Some(new_id);
                    unsafe { gl::UseProgram(new_id) };
                }
            },
            None => {
                debug!("[{}]: use", new_id);

                self.id = Some(new_id);
                unsafe { gl::UseProgram(new_id) };
            }
        }

        self
    }

    pub fn done(&mut self) {
        if let Some(used_id) = self.id {
            debug!("[{}]: unuse", used_id);
            unsafe { gl::UseProgram(0) };
        }

        self.id = None;
    }
}

impl Drop for StateProgram {
    fn drop(&mut self) {
        self.done();
    }
}
