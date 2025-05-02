use crate::{
    ActiveTexture, AttributeType, Buffer, Program, Shader, ShaderError, Vao, setup_attribute,
};
use nalgebra_glm::Mat4;
use std::sync::OnceLock;

// vao, vbo, ebo
static SPRITE_DATA: OnceLock<(Vao, Buffer, Buffer, Buffer)> = OnceLock::new();

pub struct Sprite<'a> {
    texture: ActiveTexture<'a>,
    shader: Program,
    texture_size: (f32, f32),
}

const VERTEX_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uv;
out vec2 tex_uv;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(position, 0.0, 1.0);
    gl_Position.z = 0.0;
    tex_uv = uv;
}
"#;

const FRAGMENT_SOURCE: &str = r#"
#version 330 core
in vec2 tex_uv;
out vec4 color;

uniform sampler2D sprite;

void main() {
    color = texture(sprite, tex_uv);
}
"#;

impl<'a> Sprite<'a> {
    pub fn new(texture: ActiveTexture<'a>, texture_size: (u32, u32)) -> Result<Self, ShaderError> {
        let fragment_shader = Shader::new(FRAGMENT_SOURCE)?;
        let vertex_shader = Shader::new(VERTEX_SOURCE)?;
        let shader = Program::new(vertex_shader, fragment_shader)?;

        shader.use_internal();
        SPRITE_DATA.get_or_init(Self::initialize_sprite_buffer);

        Ok(Self {
            texture,
            shader,
            texture_size: (texture_size.0 as f32, texture_size.1 as f32),
        })
    }

    pub fn render(&self, position: (f32, f32), transform: Mat4, scale: f32) {
        let verts = [
            position.0,
            position.1,
            position.0 + self.texture_size.0 * scale,
            position.1,
            position.0 + self.texture_size.0 * scale,
            position.1 + self.texture_size.1 * scale,
            position.0,
            position.1 + self.texture_size.1 * scale,
        ];

        self.shader.use_internal();
        let sprite_data = SPRITE_DATA.get_or_init(Self::initialize_sprite_buffer);
        sprite_data.0.bind();
        sprite_data.1.bind();
        sprite_data.1.subdata(0, &verts);

        self.shader
            .put_uniform("mvp", &transform)
            .expect("Should not fail");
        self.shader
            .put_uniform("sprite", &self.texture)
            .expect("Should not fail");

        sprite_data.2.bind();
        sprite_data
            .0
            .draw_elements(crate::DrawMode::Triangles, 6, AttributeType::u32);
    }

    fn initialize_sprite_buffer() -> (Vao, Buffer, Buffer, Buffer) {
        let vao = Vao::new();
        vao.bind();
        let vbo = Buffer::new(crate::DrawTarget::Array);
        vbo.bind();
        vbo.data_empty(
            std::mem::size_of::<f32>() * 8,
            crate::DrawUsage::DynamicDraw,
        );
        setup_attribute(0, 2, 0, 0, crate::AttributeType::f32);

        let texture_buffer = Buffer::new(crate::DrawTarget::Array);
        texture_buffer.bind();
        let tex: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        texture_buffer.data(&tex, crate::DrawUsage::StreamDraw);
        setup_attribute(1, 2, 0, 0, crate::AttributeType::f32);

        let ebo = Buffer::new(crate::DrawTarget::ElementArray);
        ebo.bind();
        ebo.data::<u32>(&[0, 1, 3, 1, 2, 3], crate::DrawUsage::StaticDraw);

        (vao, vbo, ebo, texture_buffer)
    }
}
