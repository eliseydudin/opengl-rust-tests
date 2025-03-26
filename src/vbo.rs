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
    ArrayBuffer = gl::ARRAY_BUFFER,
    AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = gl::COPY_READ_BUFFER,
    CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
    QueryBuffer = gl::QUERY_BUFFER,
    ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
    TextureBuffer = gl::TEXTURE_BUFFER,
    TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = gl::UNIFORM_BUFFER,
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
