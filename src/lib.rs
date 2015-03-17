#![feature(collections)]
#![feature(optin_builtin_traits)]
#![feature(unsafe_destructor)]

extern crate gl;
#[macro_use] extern crate log;

pub use role::{ Generator };

mod role;
pub mod gli;

pub mod shader;
pub mod context;

pub mod program;
pub mod buffer;
pub mod vertex_array;

pub mod state_program;
pub mod state_buffer;
