#[macro_use] extern crate log;
#[macro_use] extern crate imgui;
// #[macro_use] extern crate dyon;
// use rhai::{Engine, Scope, RegisterFn};
// use mun_runtime::{invoke_fn, RetryResultExt, RuntimeBuilder};

use photic::pipeline::light::IsLight;
use imgui::StyleColor;

use std::{
    os::raw::c_void,
    path::PathBuf,
};

use glow::HasContext;

use sdl2::{
    event::Event,
};

use cgmath::*;

use std::time::Instant;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::collections::HashMap;

use photic::{
    x3d::Renderer3D,
    pipeline::{
        render_mesh::RenderMesh,
        material::Material,
        light::DirectionalLight,
        shader::{Shader, ShaderSource},
        texture::Texture,
    },
};

mod ui;
// mod imgui_dyon;

pub fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::max())
        .init();

    let (mut surface, gl, _gl_context) = photic::initialize(1280, 720).expect("Failed to initialize photic!");

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    ui::style_ui(imgui.style_mut());

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &surface.window);

    gl::load_with(|s| surface.video.gl_get_proc_address(s) as _);

    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| surface.video.gl_get_proc_address(s) as *const c_void);

    let mut event_pump = surface.sdl.event_pump().expect("Failed to get event pump!");

    let mut last_frame = Instant::now();
    let mut delta_s = 0.0;

    let mut entities = tiny_ecs::Entities::new(None, None); //TODO possibly optimise this

    //Rhai
    // let path = PathBuf::from("../test.rhai");
    // let mut engine = Engine::new();
    // engine.register_type::<Arc<imgui::Ui>>();
    // let ast = engine.compile_file(path);
    // let mut scope = Scope::new();

    //Dyon
    // let mut runtime = dyon::Runtime::new();
    // let mut test_module = dyon::Module::new();
    // dyon::load("../test.dyon", &mut test_module);
    // let test_module_arc = Arc::new(test_module);

    //Mun
    // let mut runtime = RuntimeBuilder::new("../test.mun")
    //     .spawn()
    //     .expect("Failed to spawn Mun's runtime!");
    // runtime.borrow_mut().update() can detect code changes and recompile on the fly

    //Test stuff
    let mut mat_rough = 0.5;
    let mut mat_metalness = 0.0;

    let camera = photic::camera::Camera::default();

    let mut x3d = Renderer3D::<Material>::new();

    let vf_mesh = xe_vfs::VirtualFile::new("../dragon_fixed.obj".to_string()).expect("Failed to open monkey_2.8.obj!");
    let vf_texture = xe_vfs::VirtualFile::new("../white.png".to_string()).expect("Failed to open monkey_d1.png!");

    let objects = vf_mesh.get_mesh().expect("Failed to get mesh!");

    let shader_source = ShaderSource {
        vertex_shader: include_str!("../../shaders/vertex.glsl").to_string(),
        geometry_shader: None,
        tesselation_shader: None,
        fragment_shader: include_str!("../../shaders/fragment.glsl").to_string(),
    };
    let shader = Shader::from_source(shader_source);
    let texture = Texture::from_rgba_image(&gl, vf_texture.get_texture().unwrap());
    let default_material = Material::new(&shader, &texture, [1.0, 1.0, 1.0, 1.0], 0.0, 0.5);
    let mut material = Material::new(&shader, &texture, [1.0, 1.0, 1.0, 1.0], 0.0, 0.5);

    let mut mesh_rotation = 0.0;
    let mut model_matrix = {
        let translation = Matrix4::<f32>::from_translation(vec3(0.0, -0.5, -4.0));
        let rotation: Matrix4<f32> = Quaternion::<f32>::from_angle_y(Deg(mesh_rotation)).into();
        let scale = Matrix4::<f32>::from_nonuniform_scale(4.0, 4.0, 4.0);

        translation * rotation * scale
    };

    // let mut meshes: Vec<RenderMesh> = Vec::new();
    for obj in objects {
        // meshes.push(RenderMesh::new(&mut surface, false, obj.0.clone(), obj.1.clone()).unwrap());
        entities.new_entity()
                .with(model_matrix).unwrap()
                .with(RenderMesh::new(&mut surface, false, obj.0.clone(), obj.1.clone()).unwrap()).unwrap()
                .finalise().unwrap();
    }

    let meshes = entities.borrow_mut::<RenderMesh>().unwrap();

    let light_direction = Quaternion::from(Euler {
        x: Deg(15.0),
        y: Deg(-35.0),
        z: Deg(-130.0),
    });
    let light = DirectionalLight::new(light_direction, 8.0, [1.0, 1.0, 1.0]);

    debug!("Created light, shader, material and mesh!");

    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) { continue; }

            match event {
                Event::Quit{..} => {
                    break 'main;
                },
                _ => {}
            }
        }

        // camera.rotation_y += delta_s * 90.0;
        // mesh_rotation += delta_s * 45.0;
        if mesh_rotation > 360.0 { mesh_rotation = 0.0; }
        model_matrix = {
            let translation = Matrix4::<f32>::from_translation(vec3(0.0, -0.5, -4.0));
            let rotation: Matrix4<f32> = Quaternion::<f32>::from_angle_y(Deg(mesh_rotation)).into();
            let scale = Matrix4::<f32>::from_nonuniform_scale(3.5, 3.5, 3.5);

            translation * rotation * scale
        };

        imgui_sdl2.prepare_frame(imgui.io_mut(), &surface.window, &event_pump.mouse_state());

        let now = Instant::now();
        let delta = now - last_frame;
        delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;

        let ui = imgui.frame();

        let stats_window = imgui::Window::new(im_str!("Metrics"))
            .position([10.0, 10.0], imgui::Condition::Appearing)
            .size([120.0, 120.0], imgui::Condition::Appearing)
            .focused(false)
            .collapsible(true);

        stats_window.build(&ui, || {
            ui.text(format!("FPS: {:.1}", 1000.0 / (delta_s * 1000.0)));
            ui.text(format!("MS: {:.2}", delta_s * 1000.0));
            ui.text(format!("Verts: {}", x3d.vert_count));
            ui.text(format!("Lights: {}", x3d.light_count)); //light.count_ref().load(Ordering::SeqCst))
        });

        let material_window = imgui::Window::new(im_str!("Material"))
            .position([1050.0, 10.0], imgui::Condition::Appearing)
            .size([220.0, 90.0], imgui::Condition::Appearing)
            .focused(false)
            .collapsible(true);

        material_window.build(&ui, || {
            let in_rotation = ui.drag_float(im_str!("Rotation"), &mut mesh_rotation);
            in_rotation.min(0.0).max(360.0).speed(0.02).build();
            ui.separator();
            let in_rough = ui.drag_float(im_str!("Roughness"), &mut material.roughness);
            in_rough.min(0.01).max(1.0).speed(0.01).build();
            let in_metal = ui.drag_float(im_str!("Metalness"), &mut material.metalness);
            in_metal.min(0.0).max(1.0).speed(0.01).build();
        });

        unsafe {
            gl.clear_color(127.0 / 255.0, 103.0 / 255.0, 181.0 / 255.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // runtime.call_str("update", &[dyon::Variable::f64(delta_s as f64)], &test_module_arc);

        x3d.prepare_frame(&gl);
        x3d.use_light(&light);
        for (entity_id, mesh) in meshes.iter() {
            x3d.draw_mesh(&mesh, material, model_matrix);
        }
        x3d.finish_frame(&mut surface, &gl, &camera);

        imgui_sdl2.prepare_render(&ui, &surface.window);
        renderer.render(ui);

        surface.swap_buffer();
    }
}
