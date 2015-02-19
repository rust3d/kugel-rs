use gl;
use gl::types::*;

pub struct Buffer {
    id: GLuint,
}

impl Buffer {
    pub fn gen_buffers(size: usize) -> Vec<Buffer> {
        let mut ids: Vec<GLuint> = vec![0; size];

        unsafe { gl::GenBuffers(size as GLsizei, ids.as_mut_ptr()) };

        ids
            .into_iter()
            .map(|id| Buffer { id: id })
            .collect()
    }
}
