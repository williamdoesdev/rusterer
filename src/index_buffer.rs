use log::*;
use glow::*;

pub struct IndexBuffer<'a> {
    gl: &'a glow::Context,
    data: &'a [u8],
    ibo: Option<glow::NativeBuffer>,
}

impl<'a> IndexBuffer<'a> {
    pub unsafe fn new(gl: &'a glow::Context, data: &'a [u8]) -> Self {
        let ibo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data, glow::STATIC_DRAW);

        IndexBuffer {
            gl,
            data,
            ibo: Some(ibo),
        }
    }
}

impl<'a> Drop for IndexBuffer<'a> {
    fn drop(&mut self) {
        info!("Dropping vertex buffer...");
        
        unsafe {
            if let Some(ibo) = self.ibo.take() {
                self.gl.delete_buffer(ibo);
            }
        }
    }
}
