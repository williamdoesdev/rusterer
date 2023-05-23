use glow::*;
use log::*;

pub struct VertexBuffer<'a, T> {
    gl: &'a glow::Context,
    data: &'a [T],
    vbo: Option<glow::NativeBuffer>,
}

impl<'a, T: Copy + 'a> VertexBuffer<'a, T> {
    pub unsafe fn new(gl: &'a glow::Context, data: &'a [T]) -> Self {
        info!("Creating and binding vertex buffer...");

        //Make and bind buffer
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        // Convert data to u8 slice
        let len = data.len() * std::mem::size_of::<T>();
        let data_ptr = data.as_ptr() as *const u8;
        let data_u8_slice = std::slice::from_raw_parts(data_ptr, len);

        // Send to buffer
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data_u8_slice, glow::STATIC_DRAW);

        return VertexBuffer {
            gl,
            data,
            vbo: Some(vbo),
        }
    }

    pub unsafe fn bind(&self) {
        self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
    }

    pub unsafe fn unbind(&self) {
        self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
    }

}

impl<'a, T> Drop for VertexBuffer<'a, T> {
    fn drop(&mut self) {
        info!("Dropping vertex buffer...");

        unsafe {
            if let Some(vbo) = self.vbo.take() {
                self.gl.delete_buffer(vbo);
            }
        }
    }
}
