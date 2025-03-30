use std::ffi::CStr;

use crate::{
    ActiveTexture, Buffer, DrawTarget, DrawUsage, Program, Shader, ShaderError, Texture, Vao,
};

pub struct TextureAtlas {
    texture: Texture,
    texture_size: (u32, u32),
    symbol_size: (u32, u32),

    vao: Vao,
    text_buffer: Buffer,
    program: Program,
}

impl TextureAtlas {
    const VERTEX_SHADER: &CStr = cr#"
    #version 330 core
    layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
    out vec2 tex_coords;

    uniform mat4 projection;

    void main() {
        gl_Position = vec4(vertex.xy, 0.0, 1.0) * projection;
        tex_coords = vertex.zw;
    }  
    "#;

    const FRAGMENT_SHADER: &CStr = cr#"
    #version 330 core
    in vec2 tex_coords;
    out vec4 color;

    uniform sampler2D text;

    void main() {    
        color = vec4(1.0, 1.0, 1.0, texture(text, tex_coords).r);
    }  
    "#;

    pub fn new(
        texture: Texture,
        texture_size: (u32, u32),
        symbol_size: (u32, u32),
    ) -> Result<Self, ShaderError> {
        let vao = Vao::new();
        vao.bind();

        let text_buffer = Buffer::new(DrawTarget::Array);
        text_buffer.bind();
        text_buffer.data_empty(4 * 6 * size_of::<f32>(), DrawUsage::DynamicDraw);

        let vertex_shader = Shader::from_cstr(Self::VERTEX_SHADER)?;
        let fragment_shader = Shader::from_cstr(Self::FRAGMENT_SHADER)?;
        let program = Program::new(vertex_shader, fragment_shader)?;

        Ok(Self {
            texture,
            texture_size,
            symbol_size,
            vao,
            text_buffer,
            program,
        })
    }

    fn texture_position(&self, n: u32) -> (f32, f32) {
        let w = self.texture_size.0 / self.symbol_size.0;
        let offset_y = n / w;
        let offset_x = n % w;

        let offset_x = (offset_x * self.symbol_size.0) as f32 / self.texture_size.0 as f32;
        let offset_y = (offset_y * self.symbol_size.0) as f32 / self.texture_size.0 as f32;

        (offset_x, offset_y)
    }

    pub fn draw_text<F>(&self, data: F, (mut x, y): (f32, f32), ortho: nalgebra_glm::Mat4)
    where
        F: AsRef<[u8]>,
    {
        let active_texture = ActiveTexture::new(1);
        active_texture.bind_texture(&self.texture);

        self.program.use_internal();
        self.program
            .put_uniform("text", active_texture)
            .expect("The uniform name is valid and exists in the shader");
        self.program
            .put_uniform("projection", ortho)
            .expect("The uniform name is valid and exists in the shader");

        for symbol in data.as_ref().iter().map(|s| *s as u32) {
            /*
                float xpos = x + ch.Bearing.x * scale;
            float ypos = y - (ch.Size.y - ch.Bearing.y) * scale;

            float w = ch.Size.x * scale;
            float h = ch.Size.y * scale;
            // update VBO for each character
            float vertices[6][4] = {
                { xpos,     ypos + h,   0.0f, 0.0f },
                { xpos,     ypos,       0.0f, 1.0f },
                { xpos + w, ypos,       1.0f, 1.0f },

                { xpos,     ypos + h,   0.0f, 0.0f },
                { xpos + w, ypos,       1.0f, 1.0f },
                { xpos + w, ypos + h,   1.0f, 0.0f }
            };
                 */

            let (w, h) = self.texture_position(symbol);

            let vertices = [
                x,
                (y + h),
                0.0,
                0.0,
                x,
                y,
                0.0,
                1.0,
                (x + w),
                y,
                1.0,
                1.0,
                x,
                (y + h),
                0.0,
                0.0,
                (x + w),
                y,
                1.0,
                1.0,
                (x + w),
                (y + h),
                1.0,
                0.0,
            ];

            self.vao.bind();
            self.text_buffer.bind();
            self.text_buffer.subdata(0, &vertices);
            x += w;

            self.vao.draw_arrays(crate::DrawMode::Triangles, 0, 6);
        }
    }
}
