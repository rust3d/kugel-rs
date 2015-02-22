pub mod buffer_targets;
pub mod state_program;

use self::buffer_targets::BufferTargets;
use self::state_program::StateProgram;

/// GL context root.
pub struct Context {
    pub buffers: BufferTargets,
    pub program: StateProgram,
}

impl !Send for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            buffers: BufferTargets::new(),
            program: StateProgram::new(),
        }
    }
}
