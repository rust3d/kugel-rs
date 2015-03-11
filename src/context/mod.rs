use state_buffer::BufferTargets;
use state_program::StateProgram;
use vertex_array::VertexArrayState;

/// GL context root.
pub struct Context {
    pub buffers: BufferTargets,
    pub program: StateProgram,
    pub vertex_array: VertexArrayState,
}

impl !Send for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            buffers: BufferTargets::new(),
            program: StateProgram::new(),
            vertex_array: VertexArrayState::new(),
        }
    }
}
