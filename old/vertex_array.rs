use crate::vertex_layout::*;
use crate::vertex_buffer::*;
use glow::*;
use log::*;

pub struct VertexArray<'a> {
    gl: &'a glow::Context,
    vao: Option<NativeVertexArray>,
}

impl<'a> VertexArray<'a> {
    pub unsafe fn new(gl: &'a glow::Context) -> Self {
        let vao = gl.create_vertex_array().unwrap();

        VertexArray { 
            gl, 
            vao: Some(vao),
        }
    }

    pub unsafe fn add_buffer<T: Copy + 'a>(&mut self, vb: &VertexBuffer<'a, T>, layout: &VertexLayout) {
        self.gl.bind_vertex_array(self.vao);
        vb.bind();
        for (i, element) in layout.attributes.iter().enumerate() {
            let i = i as u32;
            self.gl.enable_vertex_attrib_array(i);
            self.gl.vertex_attrib_pointer_f32(i, element.components, element.dtype, element.normalized, layout.stride, element.ptr);
        }
    }

    pub unsafe fn bind(&mut self) {
        self.gl.bind_vertex_array(self.vao);
    }
}

impl<'a> Drop for VertexArray<'a> {
    fn drop(&mut self) {
        info!("Dropping vertex array...");

        unsafe {
            if let Some(vao) = self.vao.take() {
                self.gl.delete_vertex_array(vao);
            }
        }
    }
}
