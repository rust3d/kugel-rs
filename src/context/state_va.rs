use gl;
use gl::types::*;

use std::rc::Rc;

use va::VertexArray;

pub struct StateVertexArray {
    va: Option<Rc<VertexArray>>,
}

impl StateVertexArray {
    pub fn new() -> StateVertexArray {
        StateVertexArray {
            va: None
        }
    }

    pub fn bind(&mut self, vertex_array: &Rc<VertexArray>) -> Result<&mut StateVertexArray, BindVertexArrayError> {
        debug!("[{}]: bind", vertex_array.get_id());

        self.va = Some(vertex_array.clone());
        unsafe { gl::BindVertexArray(vertex_array.get_id()) };

        Ok(self)
    }

    pub fn unbind(&mut self) {
        if let Some(ref va) = self.va {
            debug!("[{}]: unbind", va.get_id());

            unsafe { gl::BindVertexArray(va.get_id()) };
        }

        self.va = None;
    }

    // --- separation of concerns?

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
}

impl Drop for StateVertexArray {
    fn drop(&mut self) {
        self.unbind();
    }
}

pub struct BindVertexArrayError;
