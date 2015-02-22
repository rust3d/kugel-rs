use gl;
use gl::types::*;

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn from_raw(id: GLuint) -> VertexArray {
        VertexArray { id: id }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    /// Returns true if contains correct vertex array object and it
    /// was bound at least once.
    pub fn is_vertex_array(&self) -> bool {
        unsafe { gl::IsVertexArray(self.id) == gl::TRUE }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        debug!("[{}]: cleanup && delete", self.id);

        unsafe { gl::DeleteVertexArrays(1, &mut self.id) };
    }
}
