use::std::fs;
use::glow::*;

mod sdl2_context;
use sdl2_context::*;
mod shader_program;
use shader_program::*;
mod vertex_buffer;
use vertex_buffer::*;
mod vertex_attribute;
use vertex_attribute::*;
mod vertex_array;
use vertex_array::*;
mod uniform;
use uniform::*;

fn main() {
    unsafe {

    let (gl, window, mut events_loop, _context) = create_sdl2_context();
        
    let shaders = vec![
    ("res/shaders/triangle.vert.glsl", glow::VERTEX_SHADER),
    ("res/shaders/triangle.frag.glsl", glow::FRAGMENT_SHADER),
    ];


    let program = NativeProgram::new_from_files(&gl, shaders);
    program.bind(&gl);

    let data = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];

    let vbo = NativeBuffer::new(&gl, &data);

    let mut layout = Vec::<VertexAttribute>::new();
    layout.push_attribute::<f32>(2);

    let vao = NativeVertexArray::new(&gl, &layout);
    vao.bind(&gl);

    let uniform = NativeUniform::new(&gl, program, "uColor");
    uniform.set([0.25, 1.0, 1.0, 1.0]);

    gl.clear_color(0.1, 0.2, 0.3, 1.0);

    'render: loop {
        {
            for event in events_loop.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'render;
                }
            }
        }

        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
        window.gl_swap_window();
    }

    // Clean up
    gl.delete_program(program);
    gl.delete_vertex_array(vao);
    gl.delete_buffer(vbo)
    }
}