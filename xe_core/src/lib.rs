#[macro_use] extern crate log;
#[macro_use] extern crate imgui;

use photic::pipeline::light::IsLight;
use imgui::StyleColor;

use glow::HasContext;

use sdl2::{
    event::Event,
    EventPump,
};

pub use cgmath::*;

use libloading::{Library, Symbol};

use std::os::raw::c_void;
use std::ops::Deref;
use std::time::Instant;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::collections::HashMap;

use photic::{
    x3d::Renderer3D,
    pipeline::{
        render_mesh::RenderMesh,
        material::{Material, IsMaterial},
        light::DirectionalLight,
        shader::{Shader, ShaderSource},
        texture::Texture,
    },
    camera::IsCamera,
};

pub use xe_vfs;
// pub use photic;

pub mod traits;
mod ui;
