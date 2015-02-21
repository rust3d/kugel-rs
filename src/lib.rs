#![feature(std_misc)]
#![feature(core)]
#![feature(collections)]
#![feature(optin_builtin_traits)]

extern crate gl;
#[macro_use] extern crate log;

pub mod shader;
pub mod program;
pub mod buffer;
pub mod vertex_array;
pub mod context;
