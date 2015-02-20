pub mod buffer_target;

/// GL context root.
pub struct Context {
    pub buffer_targets: ContextBufferTargets,
}

impl !Send for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            buffer_targets: ContextBufferTargets::new(),
        }
    }
}

pub struct ContextBufferTargets {
    pub array_buffer:           buffer_target::ArrayBufferTarget,
    pub element_array_buffer:   buffer_target::ElementArrayBufferTarget,
}

impl ContextBufferTargets {
    pub fn new() -> ContextBufferTargets {
        ContextBufferTargets {
            array_buffer: buffer_target::ArrayBufferTarget::new(),
            element_array_buffer: buffer_target::ElementArrayBufferTarget::new(),
        }
    }
}
