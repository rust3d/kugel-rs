use gl;
use gl::types::*;

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn gen_vertex_arrays(size: usize) -> Vec<VertexArray> {
        let mut ids: Vec<GLuint> = vec![0; size];

        unsafe { gl::GenVertexArrays(size as GLsizei, ids.as_mut_ptr()) };

        ids
            .into_iter()
            .map(|id| VertexArray { id: id })
            .collect()
    }

    /// Returns true if contains correct vertex array object and it
    /// was bound at least once.
    pub fn is_vertex_array(&self) -> bool {
        unsafe { gl::IsVertexArray(self.id) == gl::TRUE }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) };
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &mut self.id) };
    }
}
