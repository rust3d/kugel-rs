use gl;
use gl::types::*;

use buffer::Buffer;
use std::rc::Rc;
use std::ops::Deref;

pub trait BaseBufferTarget {
    fn get_buffer(&self) -> Option<&Buffer>;
}

macro_rules! extend_base_buffer_target {
    ( $Type:ty ) => {
        impl BaseBufferTarget for $Type {
            fn get_buffer(&self) -> Option<&Buffer> {
                match (*self).buffer {
                    Some(ref value) => Some(value),
                    None => None,
                }
            }
        }
    };
}

pub trait BindableBufferTarget {
    fn bind(&mut self, buffer: Rc<Buffer>) -> Result<(), BindBufferError>;
    fn unbind(&mut self);
}

extend_base_buffer_target!(ArrayBufferTarget);
extend_base_buffer_target!(ElementArrayBufferTarget);

pub struct ArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}

impl BindableBufferTarget for ArrayBufferTarget {
    fn bind(&mut self, buffer: Rc<Buffer>) -> Result<(), BindBufferError> {
        self.buffer = Some(buffer.clone());
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, buffer.get_id()) };
        Ok(())
    }

    fn unbind(&mut self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
        self.buffer = None;
    }
}

pub struct ElementArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}

impl BindableBufferTarget for ElementArrayBufferTarget {
    fn bind(&mut self, buffer: Rc<Buffer>) -> Result<(), BindBufferError> {
        self.buffer = Some(buffer.clone());
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.get_id()) };
        Ok(())
    }

    fn unbind(&mut self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
        self.buffer = None;
    }
}

pub struct BindBufferError;
