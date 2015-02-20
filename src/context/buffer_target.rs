use buffer::Buffer;
use std::rc::Rc;

pub struct ArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}

pub struct ElementArrayBufferTarget {
    buffer: Option<Rc<Buffer>>,
}
