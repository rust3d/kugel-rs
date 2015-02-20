use gl;
use gl::types::*;

use std::rc::Rc;
use context::buffer_target;

pub struct Buffer {
    id: GLuint,
}

impl Buffer {
    pub fn gen_buffers(size: usize) -> Vec<Buffer> {
        let mut ids: Vec<GLuint> = vec![0; size];

        unsafe { gl::GenBuffers(size as GLsizei, ids.as_mut_ptr()) };

        ids
            .into_iter()
            .map(|id| Buffer { id: id })
            .collect()
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    /// Returns true if contains correct buffer object.
    pub fn is_buffer(&self) -> bool {
        unsafe { gl::IsBuffer(self.id) == gl::TRUE }
    }
}

pub trait BindableBuffer {
    fn bind<'a, T>(&self, target: &'a mut T) -> &'a mut T where T : buffer_target::BindableBufferTarget;
}

impl BindableBuffer for Rc<Buffer> {
    fn bind<'a, T>(&self, target: &'a mut T) -> &'a mut T where T : buffer_target::BindableBufferTarget {
        target.bind(self);
        target
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.is_buffer() {
            unsafe { gl::DeleteBuffers(1, &mut self.id) };
        }
    }
}
