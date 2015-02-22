pub mod buffer_targets;

use self::buffer_targets::BufferTargets;

/// GL context root.
pub struct Context {
    pub buffers: BufferTargets,
}

impl !Send for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            buffers: BufferTargets::new(),
        }
    }
}
