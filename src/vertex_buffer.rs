use log::*;
use glow::*;

pub struct VertexBuffer<'a> {
    gl: &'a glow::Context,
    data: &'a [u8],
    vbo: Option<glow::NativeBuffer>,
}

impl<'a> VertexBuffer<'a> {
    pub unsafe fn new(gl: &'a glow::Context, data: &'a [u8]) -> Self {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);

        VertexBuffer {
            gl,
            data,
            vbo: Some(vbo),
        }
    }
}

impl<'a> Drop for VertexBuffer<'a> {
    fn drop(&mut self) {
        info!("Dropping vertex buffer...");
        
        unsafe {
            if let Some(vbo) = self.vbo.take() {
                self.gl.delete_buffer(vbo);
            }
        }
    }
}
