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
    fn bind(&mut self, buffer: &Rc<Buffer>) -> Result<(), BindBufferError>;
    fn unbind(&mut self);
}

macro_rules! impl_bindable_buffer_target {
    ( $Type:ty, $GLtarget:path ) => {
        impl BindableBufferTarget for $Type {
            fn bind(&mut self, buffer: &Rc<Buffer>) -> Result<(), BindBufferError> {
                self.buffer = Some(buffer.clone());
                unsafe { gl::BindBuffer($GLtarget, buffer.get_id()) };
                Ok(())
            }

            fn unbind(&mut self) {
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

extend_base_buffer_target!(ArrayBufferTarget);
extend_base_buffer_target!(ElementArrayBufferTarget);

impl_bindable_buffer_target!(ArrayBufferTarget, gl::ARRAY_BUFFER);
impl_bindable_buffer_target!(ElementArrayBufferTarget, gl::ELEMENT_ARRAY_BUFFER);

pub struct ArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}

impl ArrayBufferTarget {
    pub fn new() -> ArrayBufferTarget {
        ArrayBufferTarget { buffer: None }
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
