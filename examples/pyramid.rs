use gl_tests_god_save_me::*;
use image::{EncodableLayout, ImageReader};
use nalgebra_glm as glm;
use russimp::{
    mesh::Mesh,
    scene::{PostProcess, Scene},
};
use sdl2::event::{Event, WindowEvent};

fn load_pyramid() -> Result<Mesh, AnyError> {
    let scene = Scene::from_file("assets/pyramid.obj", vec![PostProcess::Triangulate])?;
    Ok(scene.meshes.into_iter().nth(0).unwrap())
}

const VERTEX_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv;
out vec3 fragment_color;
out vec2 frag_uv;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(position, 1.0);
    fragment_color = vec3(1.0);
    frag_uv = uv;
}
"#;

const FRAGMENT_SOURCE: &str = r#"
#version 330 core
in vec3 fragment_color;
in vec2 frag_uv;
out vec4 color;

uniform sampler2D pyramid_texture;

void main() {
    color = vec4(fragment_color, 1.0) * texture(pyramid_texture, frag_uv);
}
"#;

fn main() -> Result<(), AnyError> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window("uni pyramid", 640, 480)
        .allow_highdpi()
        .opengl()
        .resizable()
        .build()?;

    let attr = video.gl_attr();
    attr.set_context_major_version(3);
    attr.set_context_minor_version(3);
    attr.set_context_profile(sdl2::video::GLProfile::Core);

    let gl_ctx = window.gl_create_context()?;
    window.gl_make_current(&gl_ctx)?;

    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    let mut events = sdl.event_pump()?;
    let timer = sdl.timer()?;

    let vao = Vao::new();
    vao.bind();

    let vertex_shader = Shader::new(VERTEX_SOURCE)?;
    let fragment_shader = Shader::new(FRAGMENT_SOURCE)?;
    let program = Program::new(vertex_shader, fragment_shader)?;
    program.use_internal();

    let mesh = load_pyramid()?;
    let mut positions = vec![];
    mesh.vertices.iter().for_each(|s| {
        positions.push(s.x);
        positions.push(s.y);
        positions.push(s.z);
    });

    let mut indices = vec![];
    mesh.faces.iter().for_each(|f| {
        for ind in &f.0 {
            indices.push(*ind);
        }
    });

    let mut texcoords = vec![];
    (0..(positions.len() / 3)).for_each(|_s| {
        texcoords.push(0.0_f32);
        texcoords.push(0.0_f32);

        texcoords.push(0.5_f32);
        texcoords.push(1.0_f32);

        texcoords.push(1.0_f32);
        texcoords.push(0.0_f32);
    });

    // let mut texcoords = vec![];
    // mesh.texture_coords[0]
    //     .as_ref()
    //     .unwrap()
    //     .iter()
    //     .for_each(|tx| {
    //         texcoords.push(tx.x);
    //         texcoords.push(tx.y);
    //     });

    let vbo = Buffer::new(DrawTarget::Array);
    vbo.bind();
    vbo.data(&positions, DrawUsage::StaticDraw);
    setup_attribute(0, 3, 0, 0, AttributeType::f32);

    let vbo_uv = Buffer::new(DrawTarget::Array);
    vbo_uv.bind();
    vbo_uv.data(&texcoords, DrawUsage::StaticDraw);
    setup_attribute(1, 2, 0, 0, AttributeType::f32);

    let ebo = Buffer::new(DrawTarget::ElementArray);
    ebo.bind();
    ebo.data(&indices, DrawUsage::StaticDraw);

    let indices = indices.len() as i32;

    enable_depth();
    set_clear_color(
        0x13 as f32 / 255.0,
        0x17 as f32 / 255.0,
        0x21 as f32 / 255.0,
        1.0,
    );

    let mut camera = camera::Camera::new((640, 480));
    camera.position = glm::vec3(1.0, 2.0, 0.0);

    // TEXTURE :D
    let image = ImageReader::open("assets/uni.png")?
        .decode()?
        .flipv()
        .to_rgba8();

    let texture = Texture::new(
        image.as_bytes(),
        image.width() as i32,
        image.height() as i32,
    );

    let mut active_texture = ActiveTexture::new(0);
    active_texture.bind_texture(&texture);
    program.put_uniform("pyramid_texture", &active_texture)?;

    let mut model = glm::Mat4::from_fn(|i, j| if i == j { 1.0 } else { 0.0 });
    let mut time_prev = timer.ticks64();

    'running: loop {
        let time_current = timer.ticks64();
        let delta = (time_current - time_prev) as f32;
        time_prev = time_current;

        model = glm::rotate(
            &model,
            (delta / 25.0).to_radians(),
            &glm::vec3(1.0, 0.0, 0.0),
        );

        let mvp = camera.calculate_projection() * camera.calculate_view() * model;
        program.put_uniform("mvp", &mvp)?;
        //program.put_uniform("model", model)?;
        //program.put_uniform("view", view)?;
        //program.put_uniform("projection", projection)?;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(x, y) = win_event {
                        camera.window_size = (x as u32, y as u32);
                        resize_viewport((x, y));
                    }
                }
                _ => (),
            }
        }

        clear(ClearFlags::COLOR | ClearFlags::DEPTH);
        vao.draw_elements(DrawMode::Triangles, indices, AttributeType::u32);
        window.gl_swap_window();
        timer.delay(1);
    }

    Ok(())
}
