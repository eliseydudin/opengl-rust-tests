use core::{ffi::c_void, mem, ptr};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum DrawUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum DrawTarget {
    Array = gl::ARRAY_BUFFER,
    AtomicCounter = gl::ATOMIC_COUNTER_BUFFER,
    CopyRead = gl::COPY_READ_BUFFER,
    CopyWrite = gl::COPY_WRITE_BUFFER,
    DispatchIndirect = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirect = gl::DRAW_INDIRECT_BUFFER,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER,
    PixelPack = gl::PIXEL_PACK_BUFFER,
    PixelUnpack = gl::PIXEL_UNPACK_BUFFER,
    Query = gl::QUERY_BUFFER,
    ShaderStorage = gl::SHADER_STORAGE_BUFFER,
    Texture = gl::TEXTURE_BUFFER,
    TransformFeedback = gl::TRANSFORM_FEEDBACK_BUFFER,
    Uniform = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[allow(unused)]
pub enum DrawMode {
    Triangles = gl::TRIANGLES,
    /* TODO */
}

#[derive(Debug)]
pub struct Buffer {
    id: u32,
    target: DrawTarget,
}

impl Buffer {
    pub fn new(target: DrawTarget) -> Self {
        let id = unsafe {
            let mut ptr = 0_u32;
            gl::GenBuffers(1, ptr::addr_of_mut!(ptr));
            ptr
        };

        Self { id, target }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target as u32, self.id) }
    }

    pub fn data<T>(&self, data: &[T], draw_type: DrawUsage) {
        unsafe {
            gl::BufferData(
                self.target as u32,
                (mem::size_of_val(data)) as isize,
                data.as_ptr() as *const c_void,
                draw_type as u32,
            )
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, ptr::addr_of!(self.id)) }
    }
}
