use crate::Uniform;
use std::{cell::Cell, ffi::c_void, fmt::Debug, ptr};

pub struct Texture(u32);

impl Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture").field("id", &self.0).finish()
    }
}

impl Texture {
    /// Create a new texture with the given bytes.
    /// Uses the RGBA format internally.
    pub fn new(data: &[u8], width: i32, height: i32) -> Self {
        let mut id = 0_u32;

        unsafe {
            gl::GenTextures(1, ptr::addr_of_mut!(id));
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self(id)
    }

    /// Bind a texture to an index
    unsafe fn bind(&self, index: u32) {
        assert!(
            index < 32,
            "The texture index goes outside of the maximum texture range"
        );

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, self.0)
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, ptr::addr_of!(self.0)) }
    }
}

pub struct ActiveTexture {
    id: u32,
    texture_bound: Cell<bool>,
}

impl Debug for ActiveTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActiveTexture")
            .field("id", &self.id)
            .field("texture_bound", &self.texture_bound.get())
            .finish()
    }
}

impl ActiveTexture {
    pub fn new(index: u32) -> Self {
        assert!(
            index < 32,
            "The texture index goes outside of the maximum texture range"
        );
        Self {
            id: index,
            texture_bound: Cell::new(false),
        }
    }

    pub fn bind_texture(&self, texture: &Texture) {
        unsafe { texture.bind(self.id) };
        self.texture_bound.set(true)
    }
}

impl Uniform for ActiveTexture {
    unsafe fn put_uniform(&self, location: i32) {
        assert!(
            self.texture_bound.get(),
            "No texture has been bound to this ActiveTexture"
        );

        unsafe { gl::Uniform1i(location, self.id as i32) }
    }
}
