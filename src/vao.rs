use crate::AttributeType;

use super::DrawMode;
use core::ptr;
use std::ptr::null;

#[derive(Debug)]
pub struct Vao {
    id: u32,
}

impl Vao {
    pub fn new() -> Self {
        let id = unsafe {
            let mut ptr = u32::default();
            gl::GenVertexArrays(1, ptr::addr_of_mut!(ptr));
            ptr
        };

        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        self.bind();
        unsafe { gl::DrawArrays(mode as u32, first, count) }
    }

    pub fn draw_elements(&self, mode: DrawMode, count: i32, atype: AttributeType) {
        self.bind();
        unsafe { gl::DrawElements(mode as u32, count, atype as u32, null()) }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, ptr::addr_of!(self.id)) }
    }
}
