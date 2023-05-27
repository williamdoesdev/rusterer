use::glow::*;

mod sdl2_context;
use sdl2_context::*;
mod shader_program;
use shader_program::*;
mod vertex_attribute;
use vertex_attribute::*;
mod vertex_array;
use vertex_array::*;
mod uniform;
use uniform::*;
mod renderer;
use renderer::*;
mod vertex_buffer;
use vertex_buffer::*;
mod index_buffer;
use index_buffer::*;

fn main() {
    unsafe {

    let (gl, window, mut events_loop, _context) = create_sdl2_context();
        
    let shaders = vec![
    ("res/shaders/triangle.vert.glsl", glow::VERTEX_SHADER),
    ("res/shaders/triangle.frag.glsl", glow::FRAGMENT_SHADER),
    ];


    let program = NativeProgram::new_from_files(&gl, shaders);
    program.bind(&gl);

    let vertices = [
        -0.5f32, -0.5f32, 
        0.5f32, -0.5f32, 
        0.5f32, 0.5f32, 
        -0.5f32, 0.5f32];
    let vertex_buffer = VertexBuffer::new(&gl, &vertices);

    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];
    let index_buffer = IndexBuffer::new(&gl, &indices);

    let mut layout = Vec::<VertexAttribute>::new();
    layout.push_attribute::<f32>(2);

    let vertex_array = NativeVertexArray::new(&gl, &layout);
    vertex_array.bind(&gl);

    let uniform = Uniform::new(&gl, program, "uColor");
    uniform.set([0.25, 1.0, 1.0, 1.0]);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    let renderer = Renderer::new(&gl);

    'render: loop {
        {
            for event in events_loop.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'render;
                }
            }
        }

        renderer.draw(&vertex_buffer, &vertex_array, &index_buffer, &program);
        window.gl_swap_window();
    }

    println!("glError: {}", gl.get_error());

    gl.delete_program(program);
    gl.delete_vertex_array(vertex_array);
    }
}