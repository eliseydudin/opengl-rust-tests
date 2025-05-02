use gl_tests_god_save_me::*;
use image::{EncodableLayout, ImageReader};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Scancode,
};

fn main() -> Result<(), AnyError> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window("sprite", 640, 480)
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

    let mut camera = Camera::new((640, 480));

    set_clear_color(
        0x37 as f32 / 255.0,
        0x69 as f32 / 255.0,
        0x96 as f32 / 255.0,
        1.0,
    );

    let image = ImageReader::open("assets/livekohazereaction.png")?
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
    let sprite = Sprite::new(active_texture, (image.width(), image.height()))?;
    let mut scale = 1.0;

    let timer = sdl.timer()?;
    let mut current_time = timer.ticks();

    'running: loop {
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

        let new_time = timer.ticks();
        let delta = (new_time - current_time) as f32 / 1000.0;
        current_time = new_time;

        clear(ClearFlags::COLOR);
        sprite.render((100.0, 100.0), camera.calculate_projection_ortho(), scale);
        window.gl_swap_window();

        if events.keyboard_state().is_scancode_pressed(Scancode::Q) {
            scale -= 0.5 * delta;
        } else if events.keyboard_state().is_scancode_pressed(Scancode::E) {
            scale += 0.5 * delta;
        }
    }

    Ok(())
}
