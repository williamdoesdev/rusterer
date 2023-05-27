use glow::*;
use crate::vertex_array::*;
use crate::shader_program::*;
use crate::index_buffer::*;
use crate::vertex_buffer::*;

pub struct Renderer<'a> {
    gl: &'a glow::Context
}

impl<'a> Renderer<'a> {
    pub fn new(gl: &'a glow::Context) -> Self{
        return Renderer { gl: gl }
    }

    pub fn draw<T: Copy, U: Copy>(&self, vb: &VertexBuffer<'_, T>, va: &glow::NativeVertexArray, ib: &IndexBuffer<'_, U>, program: &glow::NativeProgram){
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            vb.bind();
            va.bind(self.gl);
            ib.bind();
            program.bind(self.gl);

            self.gl.draw_elements(glow::TRIANGLES, ib.len, glow::UNSIGNED_INT, 0);
        }
    }
}