pub mod buffer_target;

/// GL context root.
struct Context {
    buffer_targets: ContextBufferTargets,
}

impl !Send for Context {}

pub struct ContextBufferTargets {
    pub array_buffer:           buffer_target::ArrayBufferTarget,
    pub element_array_buffer:   buffer_target::ElementArrayBufferTarget,
}
