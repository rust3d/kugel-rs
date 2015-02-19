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
}
