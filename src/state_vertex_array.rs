use gl;
use gl::types::*;

use std::fmt;

use vertex_array::VertexArray;

pub struct ContextVertexArray;

impl ContextVertexArray {
    pub fn new() -> ContextVertexArray {
        ContextVertexArray
    }

    pub fn gen_one(&self) -> VertexArray {
        debug!("gen, size = one");

        let mut id = 0;

        unsafe { gl::GenVertexArrays(1, &mut id) };

        debug!("[{}]: generated", id);

        VertexArray::from_raw(id)
    }

    pub fn gen(&self, size: usize) -> Vec<VertexArray> {
        debug!("gen, size = {}", size);

        let mut ids: Vec<GLuint> = vec![0; size];

        unsafe { gl::GenVertexArrays(size as GLsizei, ids.as_mut_ptr()) };

        debug!(
            "[{}]: generated",
            ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .connect(", ")
        );

        ids
            .into_iter()
            .map(|id| VertexArray::from_raw(id))
            .collect()
    }

    pub fn bind<'a>(&'a mut self, vertex_array: &'a mut VertexArray) -> BoundVertexArray {
        BoundVertexArray::new(self, vertex_array)
    }
}

pub struct BoundVertexArray<'a> {
    cva: &'a mut ContextVertexArray,
    va: &'a mut VertexArray,
}

impl<'a> BoundVertexArray<'a> {
    pub fn new(context_va: &'a mut ContextVertexArray, vertex_array: &'a mut VertexArray) -> BoundVertexArray<'a> {
        let mut fresh = BoundVertexArray::<'a> {
            cva: context_va,
            va: vertex_array,
        };

        fresh.bind();
        fresh
    }

    fn bind(&mut self) {
        let new_id = self.va.get_id();

        debug!("[{}]: bind", new_id);
        unsafe { gl::BindVertexArray(new_id) };
    }

    pub fn enable_attrib(&mut self, index: GLuint) {
        debug!("[{}]: enable attrib, index = {}", self.va.get_id(), index);
        unsafe { gl::EnableVertexAttribArray(index) };
    }

    pub fn disable_attrib(&self, index: GLuint) {
        debug!("[{}]: disable attrib, index = {}", self.va.get_id(), index);
        unsafe { gl::DisableVertexAttribArray(index) };
    }
}

#[unsafe_destructor]
impl<'a> Drop for BoundVertexArray<'a> {
    fn drop(&mut self) {
        debug!("[{}]: unbind", self.va.get_id());
        unsafe { gl::BindVertexArray(0) };
    }
}
