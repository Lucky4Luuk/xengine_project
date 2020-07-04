///Big idea: load editor as dll and load usercode as dll, don't fuck around with dll's outside of that lol

#[macro_use] extern crate log;
#[macro_use] extern crate imgui;

use xe_core::{
    EngineCore,
    xe_vfs,
    photic::{
        pipeline::{
            render_mesh::RenderMesh,
            material::{Material, IsMaterial},
            light::DirectionalLight,
            shader::{Shader, ShaderSource},
            texture::Texture,
        },
        camera::Camera,
    },

    Matrix4,
    Quaternion,
    vec3,
    Rotation3,
    Deg, Rad,
    Euler,
};

use sdl2::event::Event;

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::max())
        .init();

    debug!("Hello, editor!");

    let mut engine = EngineCore::new();

    debug!("Loaded engine core!");
    debug!("| Version: {:?}", engine.version());

    let vf_mesh = xe_vfs::VirtualFile::new("../dragon_fixed.obj".to_string()).expect("Failed to open monkey_2.8.obj!");
    let vf_texture = xe_vfs::VirtualFile::new("../white.png".to_string()).expect("Failed to open monkey_d1.png!");

    let vf_vertex = xe_vfs::VirtualFile::new("../shaders/vertex.glsl".to_string()).expect("Failed to open `shaders/vertex.glsl`!");
    let vf_fragment = xe_vfs::VirtualFile::new("../shaders/fragment.glsl".to_string()).expect("Failed to open `shaders/vertex.glsl`!");

    let objects = vf_mesh.get_mesh().expect("Failed to get mesh!");

    let shader_source = ShaderSource {
        vertex_shader: vf_vertex.read().unwrap(),
        geometry_shader: None,
        tesselation_shader: None,
        fragment_shader: vf_fragment.read().unwrap(),
    };
    let shader = Shader::from_source(shader_source);
    let texture = Texture::from_rgba_image(&engine.gl, vf_texture.get_texture().unwrap());
    let default_material = Material::new(&shader, &texture, [1.0, 1.0, 1.0, 1.0], 0.0, 0.5);
    let mut material = Material::new(&shader, &texture, [1.0, 1.0, 1.0, 1.0], 0.0, 0.5);

    let mut mesh_rotation = 0.0;
    let mut model_matrix = {
        let translation = Matrix4::<f32>::from_translation(vec3(0.0, -0.5, -4.0));
        let rotation: Matrix4<f32> = Quaternion::<f32>::from_angle_y(Deg(mesh_rotation)).into();
        let scale = Matrix4::<f32>::from_nonuniform_scale(4.0, 4.0, 4.0);

        translation * rotation * scale
    };

    let mut last_id = 0;

    for obj in objects {
        // meshes.push(RenderMesh::new(&mut surface, false, obj.0.clone(), obj.1.clone()).unwrap());
        last_id = engine.entities.new_entity()
                .with(model_matrix).unwrap()
                .with(RenderMesh::new(&mut engine.surface, false, obj.0.clone(), obj.1.clone()).unwrap()).unwrap()
                .finalise().unwrap();
    }

    let light_direction = Quaternion::from(Euler {
        x: Deg(15.0),
        y: Deg(-35.0),
        z: Deg(-130.0),
    });
    let light = DirectionalLight::new(light_direction, 8.0, [1.0, 1.0, 1.0]);

    let camera = Camera::default();

    'main: loop {
        let events = engine.prepare_frame();

        for event in events {
            match event {
                Event::Quit{..} => {
                    break 'main;
                },
                _ => {}
            }
        }

        engine.x3d.prepare_frame(&engine.gl);

        engine.x3d.use_light(&light);

        engine.x3d.finish_frame(&mut engine.surface, &engine.gl, &camera);

        let ui = engine.imgui.frame();

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

        let imgui_renderer = engine.imgui_renderer.render(ui);

        engine.end_frame();
    }
}
