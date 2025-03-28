use gl_tests_god_save_me::*;

const TRIANGLE_DATA: [f32; 15] = [
    0.0, 0.5, // first pos
    1.0, 0.819, 0.729, // first color
    0.5, -0.5, // second pos
    0.807, 0.490, 0.647, // second color
    -0.5, -0.5, // third pos
    0.745, 0.8980, 0.749, // third color
];

const VERTEX_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 fragment_color;

uniform float time;

void main() {
    gl_Position = vec4(position * cos(time), 1.0, 1.0);
    fragment_color = color;
}
"#;

const FRAGMENT_SOURCE: &str = r#"
#version 330 core
in vec3 fragment_color;
out vec4 color;

uniform float time;

void main() {
    color = vec4(fragment_color * (sin(time) + 0.5), 1.0);
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
    set_clear_color(0.3, 0.8, 1.0, 1.0);

    let vao = Vao::new();
    vao.bind();
    let buffer = Buffer::new(DrawTarget::Array);
    buffer.bind();
    buffer.data(&TRIANGLE_DATA, DrawUsage::StaticDraw);

    let vertex_shader = Shader::new(VERTEX_SOURCE)?;
    let fragment_shader = Shader::new(FRAGMENT_SOURCE)?;
    let program = Program::new(vertex_shader, fragment_shader)?;
    program.use_internal();

    vao.bind();
    setup_attribute(0, 2, 0, 5, AttributeType::f32);
    setup_attribute(1, 3, 2, 5, AttributeType::f32);

    'running: loop {
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => (),
            }
        }

        clear(ClearFlags::COLOR);
        vao.draw_arrays(buffer::DrawMode::Triangles, 0, 3);
        window.gl_swap_window();

        let time = timer.ticks() as f32 / 500.0;
        program.put_uniform("time", time)?;
    }

    Ok(())
}
