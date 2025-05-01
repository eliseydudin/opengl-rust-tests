use crate::{ActiveTexture, Buffer, Program, Shader, ShaderError, Vao, setup_attribute};
use nalgebra_glm::Mat4;

pub struct SpriteSheet<'a> {
    texture: ActiveTexture<'a>,

    texture_w: f32,
    texture_h: f32,

    symbol_w: f32,
    symbol_h: f32,

    vao: Vao,
    buffer_verts: Buffer,
    buffer_tex: Buffer,
    ebo: Buffer,

    program: Program,
}

impl<'a> SpriteSheet<'a> {
    const VERTEX_SHADER: &'static str = "
    #version 330 core

    layout (location = 0) in vec2 position;
    layout (location = 1) in ivec2 tex_coords;
    out vec2 frag_tex_coords;
    uniform mat4 mvp;

    void main() {
        gl_Position = mvp * vec4(position, 0.0, 1.0);
        gl_Position.z = 0.0;
        frag_tex_coords = vec2(tex_coords);
    }
    ";
    const FRAGMENT_SHADER: &'static str = "
    #version 330 core
    in vec2 frag_tex_coords;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texelFetch(tex, ivec2(frag_tex_coords), 0);
    }
    ";

    pub fn new(
        texture: ActiveTexture<'a>,
        texture_size: (f32, f32),
        symbol_size: (f32, f32),
    ) -> Result<Self, ShaderError> {
        let vao = Vao::new();
        vao.bind();

        let vertex_shader = Shader::new(Self::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(Self::FRAGMENT_SHADER)?;
        let program = Program::new(vertex_shader, fragment_shader)?;
        program.use_internal();

        let buffer_verts = Buffer::new(crate::DrawTarget::Array);
        buffer_verts.bind();
        buffer_verts.data_empty(8 * size_of::<f32>(), crate::DrawUsage::DynamicDraw);
        setup_attribute(0, 2, 0, 0, crate::AttributeType::f32);

        let buffer_tex = Buffer::new(crate::DrawTarget::Array);
        buffer_tex.bind();
        buffer_tex.data_empty(8 * size_of::<i32>(), crate::DrawUsage::DynamicDraw);
        setup_attribute(1, 2, 0, 0, crate::AttributeType::i32);

        let ebo = Buffer::new(crate::DrawTarget::ElementArray);
        ebo.bind();
        ebo.data(&[0_u32, 1, 3, 1, 2, 3], crate::DrawUsage::StaticDraw);

        Ok(Self {
            texture,
            texture_w: texture_size.0,
            texture_h: texture_size.1,
            symbol_w: symbol_size.0,
            symbol_h: symbol_size.1,
            vao,
            buffer_tex,
            buffer_verts,
            program,
            ebo,
        })
    }

    pub fn draw_nth(&self, position: (f32, f32), nth: u8, mvp: Mat4, scale: f32) {
        /*
        const float verts[] = {
            posX, posY,
            posX + spriteWidth, posY,
            posX + spriteWidth, posY + spriteHeight,
            posX, posY + spriteHeight
        };
        const float tw = float(spriteWidth) / texWidth;
        const float th = float(spriteHeight) / texHeight;
        const int numPerRow = texWidth / spriteWidth;
        const float tx = (frameIndex % numPerRow) * tw;
        const float ty = (frameIndex / numPerRow + 1) * th;
        const float texVerts[] = {
            tx, ty,
            tx + tw, ty,
            tx + tw, ty + th,
            tx, ty + th
        };

        // ... Bind the texture, enable the proper arrays

        glVertexPointer(2, GL_FLOAT, verts);
        glTexCoordPointer(2, GL_FLOAT, texVerts);
        glDrawArrays(GL_TRI_STRIP, 0, 4);
         */

        self.vao.bind();
        self.program.use_internal();

        let verts = [
            position.0,
            position.1,
            position.0 + self.symbol_w * scale,
            position.1,
            position.0 + self.symbol_w * scale,
            position.1 + self.symbol_h * scale,
            position.0,
            position.1 + self.symbol_h * scale,
        ];

        self.buffer_verts.bind();
        self.buffer_verts.subdata(0, &verts);

        let num_per_row = self.texture_w / self.symbol_w;

        let tx = nth % num_per_row as u8 * self.symbol_w as u8;
        let ty = (nth / num_per_row as u8) * self.symbol_h as u8;
        let ty = (self.texture_h - ty as f32 - self.symbol_h) as u8;
        //ty -= 1 * self.symbol_h as u8;

        let tex_verts = [
            tx as i32,
            ty as i32,
            tx as i32 + self.symbol_w as i32,
            ty as i32,
            tx as i32 + self.symbol_w as i32,
            ty as i32 + self.symbol_h as i32,
            tx as i32,
            ty as i32 + self.symbol_h as i32,
        ];

        self.buffer_tex.bind();
        self.buffer_tex.subdata(0, &tex_verts);

        self.program
            .put_uniform("tex", &self.texture)
            .expect("Should not fail");

        self.program
            .put_uniform("mvp", &mvp)
            .expect("Should not fail");

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        self.ebo.bind();
        self.vao
            .draw_elements(crate::DrawMode::Triangles, 6, crate::AttributeType::u32);
    }

    pub fn draw_several(
        &self,
        mut start_pos: (f32, f32),
        several: impl AsRef<[u8]>,
        mvp: Mat4,
        scale: f32,
    ) {
        let several = several.as_ref();

        for char in several.iter() {
            self.draw_nth(start_pos, *char, mvp, scale);
            start_pos.0 += self.symbol_w * scale;
        }
    }
}
