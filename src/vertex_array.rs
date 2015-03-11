use gl;
use gl::types::*;
use std::mem;

/// Generates, binds and unbinds vertex array objects.
pub struct VertexArrayState {
    is_bound: bool,
}

/// Manipulates OpenGL vertex array object.
pub struct VertexArray {
    id: GLuint,
}

/// Manipulates OpenGL vertex array object when it is bound.
pub struct VertexArrayBound {
    va: VertexArray,
}

impl VertexArrayState {
    pub fn new() -> VertexArrayState {
        VertexArrayState {
            is_bound: false,
        }
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

    /// Create new vertex array object bound and bind specified `vertex_array`.
    ///
    /// ## glBindVertexArray
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    pub fn take_bound(&mut self, vertex_array: VertexArray) -> VertexArrayBound {
        match self.is_bound {
            false => {
                self.is_bound = true;
                VertexArrayBound::new(vertex_array)
            },
            true => {
                error!("[{}]: unreleased bind", vertex_array.get_id());
                panic!("Can not bind multiple VertexArray objects to OpenGL state.");
            }
        }
    }

    /// Unbind vertex array object and return it.
    ///
    /// ## glBindVertexArray(0)
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    pub fn end_bound(&mut self, bound: VertexArrayBound) -> VertexArray {
        debug!("[{}]: unbind", bound.va.get_id());
        unsafe { gl::BindVertexArray(0) };

        self.is_bound = false;

        bound.va
    }
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

impl VertexArrayBound {
    fn new(vertex_array: VertexArray) -> VertexArrayBound {
        let mut fresh = VertexArrayBound {
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

    /// Bind another `vertex_array` and return currently bound array.
    ///
    /// ## glBindVertexArray
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    pub fn replace(&mut self, mut vertex_array: VertexArray) -> VertexArray {
        mem::swap(&mut self.va, &mut vertex_array);

        self.bind();

        vertex_array
    }

    /// Enable a generic vertex attribute array.
    ///
    /// Specifies the `index` of the generic vertex attribute to be enabled.
    ///
    /// ## glEnableVertexAttribArray
    ///
    /// Worse alternative for `VertexArray::enable_attrib`.
    ///
    /// - OpenGL Version 2.0
    /// - OpenGL ES Version 2.0
    ///
    pub fn enable_attrib(&mut self, index: GLuint) {
        debug!("[{}]: enable attrib, index = {}", self.va.get_id(), index);
        unsafe { gl::EnableVertexAttribArray(index) };
    }

    /// Disable a generic vertex attribute array.
    ///
    /// Specifies the `index` of the generic vertex attribute to be disabled.
    ///
    /// ## glDisableVertexAttribArray
    ///
    /// Worse alternative for `VertexArray::disable_attrib`.
    ///
    /// - OpenGL Version 2.0
    /// - OpenGL ES Version 2.0
    ///
    pub fn disable_attrib(&self, index: GLuint) {
        debug!("[{}]: disable attrib, index = {}", self.va.get_id(), index);
        unsafe { gl::DisableVertexAttribArray(index) };
    }
}

impl Drop for VertexArray {

    /// Delete vertex array objects.
    ///
    /// ## glDeleteVertexArrays
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    fn drop(&mut self) {
        debug!("[{}]: cleanup && delete", self.id);
        unsafe { gl::DeleteVertexArrays(1, &mut self.id) };
    }
}

impl Drop for VertexArrayState {

    /// Cleanup state and unbind vertex array object if it is still bound.
    ///
    /// ## glBindVertexArray(0)
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    fn drop(&mut self) {
        if self.is_bound {
            debug!("[?]: unbind last");
            unsafe { gl::BindVertexArray(0) };
        }
    }
}
