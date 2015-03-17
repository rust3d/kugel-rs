extern crate kugel;
extern crate env_logger;

mod support;

use support::*;
use support::gl;
use support::gl::types::*;

use std::rc::Rc;
use std::ptr;

use kugel::*;

static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

fn main() {
    env_logger::init().ok().expect("Failed to initialize logging.");

    let mut window = Window::new(
        WindowOptions {
            gl_version: GLVersion::Core((4, 1)),
            title: "GL Triangle".to_string(),
            initial_size: (800, 600),
        }
    );

    let mut cx = context::Context::new();

    let vs = match shader::Shader::compile_vertex_shader(include_str!("gl_triangle.vert")) {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    let fs = match shader::Shader::compile_fragment_shader(include_str!("gl_triangle.frag")) {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    let mut program = match program::Program::link_new(&[Rc::new(vs), Rc::new(fs)]) {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    let vao = cx.vertex_array.gen_one();
    let vbo = cx.buffers.gen_one();

    program.bind_frag_data_location(0, "out_color");

    if let Ok(pos) = program.get_attrib_location("position") {
        let mut bound_vao = cx.vertex_array.bind(&vao);
        bound_vao.enable_attrib(pos);

        cx.buffers.array.bind(&vbo);
        cx.buffers.array.buffer_data(&VERTEX_DATA, gl::STATIC_DRAW);

        unsafe {
            gl::VertexAttribPointer(
                pos, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null()
            );
        }

        cx.buffers.array.unbind();
        cx.vertex_array.unbind(bound_vao);
    }

    cx.program.with_use(&mut program);

    window.run(move || {
        let bound_vao = cx.vertex_array.bind(&vao);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        cx.vertex_array.unbind(bound_vao);
    });
}
