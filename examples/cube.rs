use gl_tests_god_save_me::*;
use nalgebra_glm::{self as glm, vec3};
use sdl2::keyboard::Scancode;

const CUBE_VERTICES: &[f32] = &[
    -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
    -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
    0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5,
    -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5,
    -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
];

const CUBE_COLORS: &[f32] = &[
    0.583, 0.771, 0.014, 0.609, 0.115, 0.436, 0.327, 0.483, 0.844, 0.822, 0.569, 0.201, 0.435,
    0.602, 0.223, 0.310, 0.747, 0.185, 0.597, 0.770, 0.761, 0.559, 0.436, 0.730, 0.359, 0.583,
    0.152, 0.483, 0.596, 0.789, 0.559, 0.861, 0.639, 0.195, 0.548, 0.859, 0.014, 0.184, 0.576,
    0.771, 0.328, 0.970, 0.406, 0.615, 0.116, 0.676, 0.977, 0.133, 0.971, 0.572, 0.833, 0.140,
    0.616, 0.489, 0.997, 0.513, 0.064, 0.945, 0.719, 0.592, 0.543, 0.021, 0.978, 0.279, 0.317,
    0.505, 0.167, 0.620, 0.077, 0.347, 0.857, 0.137, 0.055, 0.953, 0.042, 0.714, 0.505, 0.345,
    0.783, 0.290, 0.734, 0.722, 0.645, 0.174, 0.302, 0.455, 0.848, 0.225, 0.587, 0.040, 0.517,
    0.713, 0.338, 0.053, 0.959, 0.120, 0.393, 0.621, 0.362, 0.673, 0.211, 0.457, 0.820, 0.883,
    0.371, 0.982, 0.099, 0.879,
];

const VERTEX_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
out vec3 fragment_color;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(position, 1.0);
    fragment_color = color;
}
"#;

const FRAGMENT_SOURCE: &str = r#"
#version 330 core
in vec3 fragment_color;
out vec4 color;

void main() {
    color = vec4(fragment_color, 1.0);
}
"#;

fn main() -> Result<(), AnyError> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window("hai :3", 640, 480)
        .allow_highdpi()
        .opengl()
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

    let buffer_vertices = Buffer::new(DrawTarget::Array);
    buffer_vertices.bind();
    buffer_vertices.data(&CUBE_VERTICES, DrawUsage::StaticDraw);
    setup_attribute(0, 3, 0, 0, AttributeType::f32);

    let buffer_colors = Buffer::new(DrawTarget::Array);
    buffer_colors.bind();
    buffer_colors.data(&CUBE_COLORS, DrawUsage::StaticDraw);
    setup_attribute(1, 3, 0, 0, AttributeType::f32);

    enable_depth();
    set_clear_color(
        0x19 as f32 / 255.0,
        0x2a as f32 / 255.0,
        0x51 as f32 / 255.0,
        1.0,
    );

    let mut camera = camera::Camera::new((640, 480));
    camera.position = glm::vec3(2.0, 2.0, 0.0);

    //let view = glm::look_at(
    //    &-glm::vec3(2.0, 2.0, 0.0),
    //    &glm::vec3(0.0, 0.0, 0.0),
    //    &glm::vec3(0.0, 1.0, 0.0),
    //);

    //let projection = glm::perspective(640.0 / 480.0, 90.0_f32.to_radians(), 0.1, 100.0);
    let model = glm::Mat4::from_fn(|i, j| if i == j { 1.0 } else { 0.0 });

    'running: loop {
        let time_current = timer.ticks64();

        let scale = (time_current as f32 / 1000.0).sin().abs();
        let model = glm::scale(&model, &vec3(scale, scale, scale));
        let model = glm::rotate(
            &model,
            (time_current as f32 / 25.0).to_radians(),
            &glm::vec3(1.0, 0.0, 0.0),
        );

        let mvp = camera.calculate_projection() * camera.calculate_view() * model;
        program.put_uniform("mvp", &mvp)?;
        //program.put_uniform("model", model)?;
        //program.put_uniform("view", view)?;
        //program.put_uniform("projection", projection)?;

        if events.keyboard_state().is_scancode_pressed(Scancode::A) {
            camera.fov += 0.1;
        } else if events.keyboard_state().is_scancode_pressed(Scancode::D) {
            camera.fov -= 0.1;
        }

        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => (),
            }
        }

        clear(ClearFlags::COLOR | ClearFlags::DEPTH);
        vao.draw_arrays(buffer::DrawMode::Triangles, 0, 36);
        window.gl_swap_window();
        timer.delay(1);
    }

    Ok(())
}
