pub mod buffer;

use gl;
use gl::types::*;

use std::rc::Rc;
use super::buffer::Buffer;

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

pub struct BufferTargets {
    pub array:          buffer::ArrayBufferTarget,
    pub element_array:  buffer::ElementArrayBufferTarget,
}

impl BufferTargets {
    pub fn new() -> BufferTargets {
        BufferTargets {
            array:          buffer::ArrayBufferTarget::new(),
            element_array:  buffer::ElementArrayBufferTarget::new(),
        }
    }

    pub fn gen_one(&self) -> Rc<Buffer> {
        let mut id = 0;

        unsafe { gl::GenBuffers(1, &mut id) };

        Rc::new(Buffer::from_raw(id))
    }

    pub fn gen(&self, size: usize) -> Vec<Rc<Buffer>> {
        let mut ids: Vec<GLuint> = vec![0; size];

        unsafe { gl::GenBuffers(size as GLsizei, ids.as_mut_ptr()) };

        ids
            .into_iter()
            .map(|id| Rc::new(Buffer::from_raw(id)))
            .collect()
    }
}
