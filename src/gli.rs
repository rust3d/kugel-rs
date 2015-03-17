use std::marker::PhantomFn;

use gl::types::*;

pub trait Generate: PhantomFn<Self> {
    fn gl_gen(size: usize) -> Vec<GLuint>;
}

pub trait IntoObject<Object> : PhantomFn<Self> {
    fn new_object(id: GLuint) -> Object;
}
