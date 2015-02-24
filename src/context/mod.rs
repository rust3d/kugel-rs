use state_buffer::BufferTargets;
use state_program::StateProgram;
use state_vertex_array::ContextVertexArray;

/// GL context root.
pub struct Context {
    pub buffers: BufferTargets,
    pub program: StateProgram,
    pub vertex_array: ContextVertexArray,
}

impl !Send for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            buffers: BufferTargets::new(),
            program: StateProgram::new(),
            vertex_array: ContextVertexArray::new(),
        }
    }
}
