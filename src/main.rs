#![allow(unused)]

use glow::*;
use std::fs;
use log::info;

mod context;
mod vertex_buffer;
mod index_buffer;
mod vertex_array;
mod vertex_layout;
mod shader;
mod renderer;

use vertex_buffer::*;
use index_buffer::*;
use vertex_array::*;
use vertex_layout::*;
use shader::*;
use renderer::*;

fn main() {
    env_logger::init();

    unsafe {
        // Create a context from a sdl2 window
        let (gl, window, mut events_loop, _context) = context::create_sdl2_context("Rusterer");
        
        // Create a shader program from source
        info!("Creating shader program...");
        let mut program = ShaderProgram::new(&gl);
        program.compile_shader("res/shaders/triangle.vert.glsl", glow::VERTEX_SHADER);
        program.compile_shader("res/shaders/triangle.frag.glsl", glow::FRAGMENT_SHADER);
        program.bind();

        // Create a vertex buffer and vertex array object
        info!("Creating vbo and vao...");
        let square_positions = [
        -0.5f32, -0.5f32,
        -0.5f32, 0.5f32,
        0.5f32, -0.5f32,
        0.5f32, 0.5f32
        ];

        // Create vb
        let vb = VertexBuffer::new(&gl, &square_positions);

        // Create va
        // TODO: Rename this, it conflicts with another struct in glow
        let mut va = vertex_array::VertexArray::new(&gl);

        // Create layout
        let mut layout = VertexLayout::new(None);
        layout.push::<f32>(2);

        // Add buffer to va
        va.add_buffer(&vb, &layout);
        // Index buffer
        let square_indices = [
        0, 1, 2,
        1, 2, 3
        ];

        let mut ib = IndexBuffer::new(&gl, &square_indices);
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // Set color uniform
        program.set_uniform("uColor", &[1.0, 1.0, 0.0, 1.0]);
        // let u_color_location = gl.get_uniform_location(program, "uColor");
        // gl.uniform_4_f32_slice(u_color_location.as_ref(), &[1.0, 1.0, 0.0, 1.0]);

        let mut green_channel: f32 = 0.0;
        let mut increment: f32 = 0.005;

        let mut renderer = Renderer::new(&gl);

        info!("Starting render loop...");
        'render: loop {
            {
                for event in events_loop.poll_iter() {
                    if let sdl2::event::Event::Quit { .. } = event {
                        break 'render;
                    }
                }
            }

            if(green_channel > 1.0){
                increment = -0.005;
            } else if (green_channel < 0.0){
                increment = 0.005;
            } 

            green_channel += increment;

            program.set_uniform("uColor", &[0.25, green_channel, 1.0, 1.0]);
            // let u_color_location = gl.get_uniform_location(program, "uColor");
            // gl.uniform_4_f32_slice(u_color_location.as_ref(), &[0.25, green_channel, 1.0, 1.0]);

            renderer.draw(&mut va, &mut ib, &mut program);

            // gl.clear(glow::COLOR_BUFFER_BIT);
            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

            window.gl_swap_window();
        }
    }
}