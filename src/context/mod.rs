pub mod buffer_targets;

use self::buffer_targets::BufferTargets;
use state_program::StateProgram;
use state_vertex_array::StateVertexArray;

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
