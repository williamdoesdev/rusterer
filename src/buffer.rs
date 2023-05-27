use glow::*;
use std::mem;
use core::fmt::Debug;

pub trait CreateFromData<T> {
    fn new(gl: &glow::Context, data: &[T], target: u32) -> Self;
}

impl<T: Copy + Debug> CreateFromData<T> for NativeBuffer {
    fn new(gl: &glow::Context, data: &[T], target: u32) -> Self {

        unsafe {
            let byte_slice = std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * mem::size_of::<T>(),
            );

            let buffer = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(target, Some(buffer));
            gl.buffer_data_u8_slice(target, byte_slice, glow::STATIC_DRAW);

            return buffer;
        };
    }
}

pub trait IsBuffer {
    fn bind(&self, gl: &glow::Context, target: u32);

    fn get_len(&self);
}

impl IsBuffer for NativeBuffer {
    fn bind(&self, gl: &glow::Context, target: u32) {
        unsafe {
            gl.bind_buffer(target, Some(*self));
        }
    }

    fn get_len(&self) {
        
    }
}