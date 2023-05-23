use log::*;
use glow::*;

pub struct IndexBuffer<'a> {
    gl: &'a glow::Context,
    pub data: &'a [u32],
    ibo: Option<glow::NativeBuffer>,
}

impl<'a> IndexBuffer<'a> {
    pub unsafe fn new(gl: &'a glow::Context, data: &'a [u32]) -> Self {
        info!("Creating and binding index buffer...");

        // Make and bind buffer
        let ibo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));

        // Convert data to u8 slice
        let len = data.len() * std::mem::size_of::<u32>();
        let data_ptr = data.as_ptr() as *const u8;
        let data_u8_slice = std::slice::from_raw_parts(data_ptr, len);

        //Send to buffer
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data_u8_slice, glow::STATIC_DRAW);

        return IndexBuffer {
            gl,
            data,
            ibo: Some(ibo),
        }
    }

    pub unsafe fn bind(&self) {
        self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ibo);
    }

    pub unsafe fn unbind(&self) {
        self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
    }
}

impl<'a> Drop for IndexBuffer<'a> {
    fn drop(&mut self) {
        info!("Dropping index buffer...");
        
        unsafe {
            if let Some(vbo) = self.ibo.take() {
                self.gl.delete_buffer(vbo);
            }
        }
    }
}
