use crate::vertex_array::VertexArray;
use crate::index_buffer::*;
use crate::shader::*;
use glow::*;

pub struct Renderer<'a> {
    pub gl: &'a glow::Context
}

impl<'a> Renderer<'a> {
    pub fn new(gl: &'a glow::Context) -> Self{
        return Renderer {
            gl: gl
        }
    }

    pub unsafe fn draw(&mut self, va: &mut VertexArray, ib: &mut IndexBuffer, shader: &mut ShaderProgram) {
        va.bind();
        ib.bind();
        shader.bind();
        shader.set_uniform("uColor", &[0.25, 0.5, 1.0, 1.0]);

        self.gl.clear(glow::COLOR_BUFFER_BIT);
        self.gl.draw_elements(glow::TRIANGLES, ib.data.len() as i32, glow::UNSIGNED_INT, 0);
    }
}