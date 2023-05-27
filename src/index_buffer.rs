use glow::*;
use std::mem;

pub struct IndexBuffer<'a, T: Copy> {
    gl: &'a glow::Context,
    ibo: glow::NativeBuffer,
    pub len: i32,
    _marker: std::marker::PhantomData<T>
}

impl<'a, T: Copy>  IndexBuffer<'a, T> {
    pub fn new(gl: &'a glow::Context, data: &[T]) -> Self {
        unsafe {
            let byte_slice = std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * mem::size_of::<T>(),
            );

            let buffer = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, byte_slice, glow::STATIC_DRAW);

            return IndexBuffer {
                gl: gl,
                ibo: buffer,
                len: data.len() as i32,
                _marker: std::marker::PhantomData,
            }
        };
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ibo))
        }
    }
}

impl<'a, T: Copy> Drop for IndexBuffer<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.ibo);
        }
    }
}