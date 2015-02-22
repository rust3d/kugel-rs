pub mod buffer_targets;
pub mod state_program;
pub mod state_va;

use self::buffer_targets::BufferTargets;
use self::state_program::StateProgram;
use self::state_va::StateVertexArray;

/// GL context root.
pub struct Context {
    pub buffers: BufferTargets,
    pub program: StateProgram,
    pub vertex_array: StateVertexArray,
}

impl !Send for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            buffers: BufferTargets::new(),
            program: StateProgram::new(),
            vertex_array: StateVertexArray::new(),
        }
    }
}
