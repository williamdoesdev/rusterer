use::std::fs;
use::glow::*;

mod sdl2_context;
use sdl2_context::*;
mod shader_program;
use shader_program::*;
mod buffer;
use buffer::*;
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

    // let vertices = [
    //     -0.5f32, -0.5f32, 
    //     0.5f32, -0.5f32, 
    //     0.5f32, 0.5f32];
    let vertices = [
        -0.5f32, -0.5f32, 
        0.5f32, -0.5f32, 
        0.5f32, 0.5f32, 
        -0.5f32, 0.5f32];
    let vertex_buffer = NativeBuffer::new(&gl, &vertices, glow::ARRAY_BUFFER);

    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];
    let index_buffer = NativeBuffer::new(&gl, &indices, glow::ELEMENT_ARRAY_BUFFER);

    let mut layout = Vec::<VertexAttribute>::new();
    layout.push_attribute::<f32>(2);

    let vertex_array = NativeVertexArray::new(&gl, &layout);
    vertex_array.bind(&gl);

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
        // gl.draw_arrays(glow::TRIANGLES, 0, 3);
        gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        window.gl_swap_window();
    }

    println!("{}", gl.get_error());

    // Clean up
    gl.delete_program(program);
    gl.delete_vertex_array(vertex_array);
    gl.delete_buffer(vertex_buffer);
    gl.delete_buffer(index_buffer);
    }
}