//bind
//add attrib from layout and buffer
//create from layout and buffer
use glow::*;
use crate::vertex_attribute::*;
use crate::vertex_buffer::*;

pub trait CreateFromLayout {
    fn new(gl: &::glow::Context, layout: &Vec<VertexAttribute>) -> NativeVertexArray;
}

impl CreateFromLayout for NativeVertexArray {
    fn new(gl: &::glow::Context, layout: &Vec<VertexAttribute>) -> NativeVertexArray {
        unsafe {
            let va = gl.create_vertex_array().expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(va));
            gl.enable_vertex_attrib_array(0);
            for (i, attribute) in layout.iter().enumerate() {
                gl.vertex_attrib_pointer_f32(i as u32, attribute.size, attribute.dtype, false, layout.stride(), attribute.offset);
            }

            return va;
        }
    }
}

pub trait BindArray {
    fn bind(&self, gl: &glow::Context);
}

impl BindArray for NativeVertexArray {
    fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(*self));
        }
    }
}