use gl;
use gl::types::*;

pub struct Buffer {
    id: GLuint,
}

impl Buffer {
    pub fn from_raw(id: GLuint) -> Buffer {
        Buffer { id: id }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    /// Returns true if contains correct buffer object.
    pub fn is_buffer(&self) -> bool {
        unsafe { gl::IsBuffer(self.id) == gl::TRUE }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        debug!("[{}]: cleanup", self.id);

        if self.is_buffer() {
            debug!("[{}]: delete", self.id);
            unsafe { gl::DeleteBuffers(1, &mut self.id) };
        }
    }
}
