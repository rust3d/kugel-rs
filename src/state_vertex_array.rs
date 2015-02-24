use gl;
use gl::types::*;

use std::rc::Rc;
use std::fmt;

use vertex_array::VertexArray;

pub struct ContextVertexArray {
    state: VertexArrayState,
}

impl ContextVertexArray {
    pub fn new() -> ContextVertexArray {
        ContextVertexArray {
            state: VertexArrayState::new()
        }
    }

    pub fn gen_one(&self) -> Rc<VertexArray> {
        debug!("gen, size = one");

        let mut id = 0;

        unsafe { gl::GenVertexArrays(1, &mut id) };

        debug!("[{}]: generated", id);

        Rc::new(VertexArray::from_raw(id))
    }

    pub fn gen(&self, size: usize) -> Vec<Rc<VertexArray>> {
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
            .map(|id| Rc::new(VertexArray::from_raw(id)))
            .collect()
    }

    pub fn bind(&mut self, vertex_array: &Rc<VertexArray>) -> &mut VertexArrayState {
        self.state.bind(vertex_array)
    }
}

pub struct VertexArrayState {
    va: Option<Rc<VertexArray>>,
}

impl VertexArrayState {
    pub fn new() -> VertexArrayState {
        VertexArrayState {
            va: None
        }
    }

    pub fn bind(&mut self, vertex_array: &Rc<VertexArray>) -> &mut VertexArrayState {
        debug!("[{}]: bind", vertex_array.get_id());

        self.va = Some(vertex_array.clone());
        unsafe { gl::BindVertexArray(vertex_array.get_id()) };

        self
    }

    pub fn unbind(&mut self) {
        if let Some(ref va) = self.va {
            debug!("[{}]: unbind", va.get_id());

            unsafe { gl::BindVertexArray(0) };
        }

        self.va = None;
    }

    pub fn enable_attrib(&self, index: GLuint) -> Result<(), VertexNotBoundError> {
        if let &Some(ref va) = &self.va {
            debug!("[{}]: enable attrib, index = {}", va.get_id(), index);

            unsafe { gl::EnableVertexAttribArray(index) };

            Ok(())
        } else {
            error!("enable attrib called for unbound vertex array");

            Err(VertexNotBoundError)
        }
    }

    pub fn disable_attrib(&self, index: GLuint) -> Result<(), VertexNotBoundError> {
        if let &Some(ref va) = &self.va {
            debug!("[{}]: disable attrib, index = {}", va.get_id(), index);

            unsafe { gl::DisableVertexAttribArray(index) };

            Ok(())
        } else {
            error!("disable attrib called for unbound vertex array");

            Err(VertexNotBoundError)
        }
    }
}

impl Drop for VertexArrayState {
    fn drop(&mut self) {
        self.unbind();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexNotBoundError;

impl fmt::Display for VertexNotBoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "No vertex array is bound.".fmt(f)
    }
}
