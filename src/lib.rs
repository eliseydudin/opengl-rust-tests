use core::ops;

pub mod buffer;
pub mod shader;
pub mod vao;

pub use {buffer::*, shader::*, vao::*};
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

pub fn clear() {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
}

pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { gl::ClearColor(r, g, b, a) }
}

pub fn enable_depth() {
    unsafe { gl::Enable(gl::DEPTH_TEST) }
}
