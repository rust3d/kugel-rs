use gl;
use gl::types::*;
use std::rc::Rc;

use gli;
use role;

struct Va { id: GLuint }
struct VaGen;

impl role::Generator for VaGen {
    type Object = Va;
}

impl gli::IntoObject<Va> for Va {
    fn new_object(id: GLuint) -> Va {
        Va { id: id }
    }
}

impl Drop for Va {
    fn drop(&mut self) {
        debug!("[{}]: cleanup && delete", self.id);
        unsafe { gl::DeleteVertexArrays(1, &mut self.id) };
    }
}

impl gli::Generate for VaGen {
    fn gl_gen(size: usize) -> Vec<GLuint> {
        let mut ids: Vec<GLuint> = vec![0; size];
        unsafe { gl::GenVertexArrays(size as GLsizei, ids.as_mut_ptr()) };
        ids
    }
}




// pub trait Gen {
//     fn gen_one(&self) -> GLuint;
//     fn gen(&self, size: usize) -> Vec<GLuint>;
// }

/// Raw vertex array object wrapper to hide RAII mechanism.
struct Raw {
    id: GLuint,
}

impl Drop for Raw {

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

/// Generates, binds and unbinds vertex array objects.
pub struct VertexArrayState {
    binding: Option<VertexArray>,
}

impl VertexArrayState {
    pub fn new() -> VertexArrayState {
        VertexArrayState {
            binding: None,
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
    pub fn bind(&mut self, vertex_array: &VertexArray) -> VertexArrayBinding {
        match self.binding {
            None => (),
            Some(ref old) => {
                error!("[{}]: can not bind {} when already bound", old.get_id(), vertex_array.get_id());
                panic!("Can not bind multiple VertexArray objects to OpenGL state.");
            }
        };

        self.binding = Some(vertex_array.clone());

        VertexArrayBinding::new(vertex_array.clone())
    }

    /// Unbind vertex array and return unbound object variant.
    ///
    /// ## glBindVertexArray(0)
    ///
    /// - OpenGL Version 3.0
    /// - OpenGL ES Version 3.0
    ///
    pub fn unbind(&mut self, _binding: VertexArrayBinding) {
        self.binding = None;
    }
}

/// Manipulates OpenGL vertex array object.
#[derive(Clone)]
pub struct VertexArray {
    raw: Rc<Raw>,
}

impl VertexArray {

    /// Create from raw name.
    pub fn from_raw(id: GLuint) -> VertexArray {
        VertexArray { raw: Rc::new(Raw { id: id }) }
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
    /// Better alternative for `VertexArrayBinding::enable_attrib`.
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
    /// Better alternative for `VertexArrayBinding::disable_attrib`.
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
pub struct VertexArrayBinding {
    va: VertexArray,
}

impl VertexArrayBinding {
    fn new(va: VertexArray) -> VertexArrayBinding {
        let binding = VertexArrayBinding {
            va : va
        };

        let new_id = binding.va.get_id();

        debug!("[{}]: bind", new_id);
        unsafe { gl::BindVertexArray(new_id) };

        binding
    }

    fn unbind(&mut self) {
        debug!("[{}]: unbind", self.va.get_id());
        unsafe { gl::BindVertexArray(0) };
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

impl Drop for VertexArrayBinding {

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
