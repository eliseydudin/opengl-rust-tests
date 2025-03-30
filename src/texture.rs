use crate::Uniform;
use std::{ffi::c_void, fmt::Debug, ptr};

pub struct Texture(u32);

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
    fn bind(&self, index: u32) {
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

impl Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture").field("id", &self.0).finish()
    }
}

#[derive(Clone)]
enum BoundTexture<'a> {
    Bound(&'a Texture),
    NotBound,
}

impl BoundTexture<'_> {
    pub fn unwrap(&self) -> &Texture {
        match self {
            Self::Bound(t) => t,
            _ => panic!("No texture present!"),
        }
    }
}

impl Debug for BoundTexture<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug: &dyn Debug = match self {
            Self::Bound(t) => *t,
            Self::NotBound => &"<not bound>",
        };

        f.debug_struct("BoundTexture")
            .field("value", debug)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct ActiveTexture<'a> {
    id: u32,
    tex: BoundTexture<'a>,
}

impl<'a> ActiveTexture<'a> {
    pub fn new(index: u32) -> Self {
        assert!(
            index < 32,
            "The texture index goes outside of the maximum texture range"
        );
        Self {
            id: index,
            tex: BoundTexture::NotBound,
        }
    }

    pub fn bind_texture(&mut self, texture: &'a Texture) {
        self.tex = BoundTexture::Bound(texture);
        texture.bind(self.id);
    }
}

impl Uniform for ActiveTexture<'_> {
    unsafe fn put_uniform(&self, location: i32) {
        self.tex.unwrap().bind(self.id);
        unsafe { gl::Uniform1i(location, self.id as i32) }
    }
}
