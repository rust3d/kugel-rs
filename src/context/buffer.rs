use gl;
use gl::types::*;

use buffer::Buffer;
use std::rc::Rc;
use std::mem;
use std::iter::ExactSizeIterator;

macro_rules! impl_bindable_buffer_target {
    ( $Type:ty, $GLtarget:path ) => {
        impl $Type {
            pub fn bind(&mut self, buffer: &Rc<Buffer>) -> Result<&mut $Type, BindBufferError> {
                self.buffer = Some(buffer.clone());
                unsafe { gl::BindBuffer($GLtarget, buffer.get_id()) };
                Ok(self)
            }

            pub fn unbind(&mut self) {
                unsafe { gl::BindBuffer($GLtarget, 0) };
                self.buffer = None;
            }
        }

        impl Drop for $Type {
            fn drop(&mut self) {
                self.unbind();
            }
        }
    };
}

impl_bindable_buffer_target!(ArrayBufferTarget, gl::ARRAY_BUFFER);
impl_bindable_buffer_target!(ElementArrayBufferTarget, gl::ELEMENT_ARRAY_BUFFER);

pub struct ArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}

impl ArrayBufferTarget {
    pub fn new() -> ArrayBufferTarget {
        ArrayBufferTarget { buffer: None }
    }

    pub fn buffer_data(&self, data: &[GLfloat], usage: GLenum) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&data[0]),
                usage
            );
        }
    }
}

pub struct ElementArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}

impl ElementArrayBufferTarget {
    pub fn new() -> ElementArrayBufferTarget {
        ElementArrayBufferTarget { buffer: None }
    }
}

pub struct BindBufferError;