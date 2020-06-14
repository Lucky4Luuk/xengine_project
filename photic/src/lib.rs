use std::{
    ffi::CString,
    os::raw::c_void,
};

#[macro_use] extern crate log;
use luminance_sdl2::SDL2Surface;

pub fn initialize(width: u32, height: u32) -> Result<(SDL2Surface, glow::Context, sdl2::video::GLContext), &'static str> {
    let surface = SDL2Surface::new(
        (4, 5), //Opengl version
        "photic",
        (width, height),
        false
    );

    match surface {
        Err(e) => {
            error!("Couldn't initialize photic!\n{}", e);
            return Err("Couldn't initialize photic!")
        },
        Ok(surface) => {
            let gl_context = surface.window.gl_create_context().expect("Couldn't create GL context");
            let gl = glow::Context::from_loader_function(|s| {
                    surface.video.gl_get_proc_address(s) as *const c_void
                });
            debug!("Photic initialized!");
            return Ok((surface, gl, gl_context));
        }
    }
}

pub mod pipeline;
pub mod camera;
pub mod x3d;

use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
    #[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    #[sem(name = "uv", repr = "[f32; 3]", wrapper = "VertexUV")]
    UV,
}

#[derive(Vertex, Copy, Clone)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
    pub normal: VertexNormal,
    pub uv: VertexUV,
    #[vertex(normalized = "true")]
    pub color: VertexRGB,
}

pub type VertexIndex = u32;
