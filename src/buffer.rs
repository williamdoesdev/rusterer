use glow::*;
use std::mem;

pub trait CreateFromData<T> {
    fn new(gl: &glow::Context, data: &[T], target: u32) -> Self;
}

impl<T: Copy> CreateFromData<T> for NativeBuffer {
    fn new(gl: &glow::Context, data: &[T], target: u32) -> Self {
        // First, we need to get a pointer to the raw bytes of the data.
        unsafe {
            let byte_slice = std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * mem::size_of::<T>(),
            );

            let vb = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(target, Some(vb));
            gl.buffer_data_u8_slice(target, byte_slice, glow::STATIC_DRAW);

            return vb;
        };
    }
}

pub trait BindBuffer {
    fn bind(&self, gl: &glow::Context, target: u32);
}

impl BindBuffer for NativeBuffer {
    fn bind(&self, gl: &glow::Context, target: u32) {
        unsafe {
            gl.bind_buffer(target, Some(*self));
        }
    }
}