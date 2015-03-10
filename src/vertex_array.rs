use gl;
use gl::types::*;

/// Manipulates OpenGL vertex array object.
pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    /// Create from raw name.
    pub fn from_raw(id: GLuint) -> VertexArray {
        VertexArray { id: id }
    }

    /// Get raw name.
    pub fn get_id(&self) -> GLuint {
        self.id
    }

    /// Enable a generic vertex attribute array.
    ///
    /// Specifies the `index` of the generic vertex attribute to be enabled.
    ///
    /// ## glEnableVertexArrayAttrib
    ///
    /// Better alternative for `VertexArrayBound::enable_attrib`.
    ///
    /// - OpenGL Version 4.5
    ///
    pub fn enable_attrib(&self, index: GLuint) {
        debug!("[{}]: enable attrib, index = {}", self.id, index);
        unsafe { gl::EnableVertexArrayAttrib(self.id, index) };
    }

    /// Disable a generic vertex attribute array.
    ///
    /// Specifies the `index` of the generic vertex attribute to be disabled.
    ///
    /// ## glDisableVertexArrayAttrib
    ///
    /// Better alternative for `VertexArrayBound::disable_attrib`.
    ///
    /// - OpenGL Version 4.5
    ///
    pub fn disable_attrib(&self, index: GLuint) {
        debug!("[{}]: disable attrib, index = {}", self.id, index);
        unsafe { gl::DisableVertexArrayAttrib(self.id, index) };
    }

    /// Determine if a name corresponds to a vertex array object.
    ///
    /// Returns true if contains correct vertex array object and it
    /// was bound at least once.
    ///
    /// ## glIsVertexArray
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
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
