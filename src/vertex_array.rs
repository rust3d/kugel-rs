use gl;
use gl::types::*;
use std::mem;
use std::rc::Rc;

/// Generates, binds and unbinds vertex array objects.
pub struct VertexArrayState {
    bound_va: Option<VertexArray>,
}

impl VertexArrayState {
    pub fn new() -> VertexArrayState {
        VertexArrayState {
            bound_va: None,
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
            .map(|id| VertexArray::from_raw(id) )
            .collect()
    }

    /// Bind vertex array object and return bound object variant.
    ///
    /// ## glBindVertexArray
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    pub fn bind(&mut self, vertex_array: &VertexArray) -> VertexArrayBound {
        match self.bound_va {
            None => (),
            Some(ref old) => {
                error!("[{}]: can not bind {} when already bound", old.get_id(), vertex_array.get_id());
                panic!("Can not bind multiple VertexArray objects to OpenGL state.");
            }
        };

        self.bound_va = Some(vertex_array.clone());

        let mut bound = VertexArrayBound {
            va: None,
        };

        bound.bind(vertex_array.clone());
        bound
    }

    /// Unbind vertex array and return unbound object variant.
    ///
    /// ## glBindVertexArray(0)
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    pub fn unbind(&mut self, mut bound: VertexArrayBound) {
        bound.unbind();

        self.bound_va = None;
    }
}

struct VertexArrayRaw {
    id: GLuint,
}

impl Drop for VertexArrayRaw {

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

/// Manipulates OpenGL vertex array object.
#[derive(Clone)]
pub struct VertexArray {
    raw: Rc<VertexArrayRaw>,
}

impl VertexArray {

    /// Create from raw name.
    pub fn from_raw(id: GLuint) -> VertexArray {
        VertexArray { raw: Rc::new(VertexArrayRaw { id: id }) }
    }

    /// Get raw name.
    pub fn get_id(&self) -> GLuint {
        self.raw.id
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
        debug!("[{}]: enable attrib, index = {}", self.get_id(), index);
        unsafe { gl::EnableVertexArrayAttrib(self.get_id(), index) };
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
        debug!("[{}]: disable attrib, index = {}", self.get_id(), index);
        unsafe { gl::DisableVertexArrayAttrib(self.get_id(), index) };
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
        unsafe { gl::IsVertexArray(self.get_id()) == gl::TRUE }
    }
}

/// Manipulates OpenGL vertex array object when it is bound.
pub struct VertexArrayBound {
    /// Optional because can be unbound.
    va: Option<VertexArray>,
}

impl VertexArrayBound {
    fn bind(&mut self, va: VertexArray) {
        let new_id = va.get_id();

        self.va = Some(va);

        debug!("[{}]: bind", new_id);
        unsafe { gl::BindVertexArray(new_id) };
    }

    fn unbind(&mut self) {
        if let Some(ref va) = self.va {
            debug!("[{}]: unbind", va.get_id());

            unsafe { gl::BindVertexArray(0) };
        }

        self.va = None;
    }

    /// Bind other `vertex_array` and return currently bound array.
    ///
    /// ## glBindVertexArray
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    // pub fn bind_other(&mut self, mut vertex_array: VertexArray) -> VertexArray {
    //     mem::swap(&mut self.va, &mut vertex_array);
    //
    //     self.bind();
    //
    //     vertex_array
    // }

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
        if let Some(ref va) = self.va {
            debug!("[{}]: enable attrib, index = {}", va.get_id(), index);
            unsafe { gl::EnableVertexAttribArray(index) };
        }
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
        if let Some(ref va) = self.va {
            debug!("[{}]: disable attrib, index = {}", va.get_id(), index);
            unsafe { gl::DisableVertexAttribArray(index) };
        }
    }
}

impl Drop for VertexArrayBound {

    /// Cleanup state and unbind vertex array object if it is still bound.
    ///
    /// ## glBindVertexArray(0)
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    fn drop(&mut self) {
        self.unbind();
    }
}
