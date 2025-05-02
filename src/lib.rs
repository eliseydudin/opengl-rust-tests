use core::ops;

pub mod buffer;
pub mod camera;
pub mod shader;
pub mod sprite;
pub mod sprite_sheet;
pub mod texture;
pub mod vao;

pub use {buffer::*, camera::*, shader::*, sprite::*, sprite_sheet::*, texture::*, vao::*};
pub type AnyError = Box<dyn std::error::Error>;

pub struct ClearFlags(u32);

impl ClearFlags {
    pub const COLOR: Self = Self(gl::COLOR_BUFFER_BIT);
    pub const DEPTH: Self = Self(gl::DEPTH_BUFFER_BIT);
    pub const STENCIL: Self = Self(gl::STENCIL_BUFFER_BIT);
}

impl ops::BitOr for ClearFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub fn clear(clear_flags: ClearFlags) {
    unsafe { gl::Clear(clear_flags.0) }
}

pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { gl::ClearColor(r, g, b, a) }
}

pub fn enable_depth() {
    unsafe { gl::Enable(gl::DEPTH_TEST) }
}

#[cfg(target_os = "macos")]
pub fn resize_viewport(window_size: (i32, i32)) {
    unsafe { gl::Viewport(0, 0, window_size.0 * 2, window_size.1 * 2) }
}

#[cfg(not(target_os = "macos"))]
pub fn resize_viewport(window_size: (i32, i32)) {
    unsafe { gl::Viewport(0, 0, window_size.0, window_size.1) }
}
