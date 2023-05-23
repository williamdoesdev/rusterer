use::std::fs;
use::glow::*;

mod sdl2_context;
use sdl2_context::*;
mod shader_program;
use shader_program::*;
mod vertex_buffer;
use vertex_buffer::*;

fn main() {
    unsafe {

    let (gl, window, mut events_loop, _context) = create_sdl2_context();

    let shaders = vec![
    ("res/shaders/triangle.vert.glsl", glow::VERTEX_SHADER),
    ("res/shaders/triangle.frag.glsl", glow::FRAGMENT_SHADER),
    ];

    let program = NativeProgram::new_from_files(&gl, shaders);
    program.bind(&gl);

    // Create a vertex buffer and vertex array object
    let (vbo, vao) = create_vertex_buffer(&gl);

    // Upload some uniforms
    set_uniform(&gl, program, "blue", 0.8);

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

unsafe fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec2s.
    let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
    let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
        triangle_vertices.as_ptr() as *const u8,
        triangle_vertices.len() * core::mem::size_of::<f32>(),
    );

    // We construct a buffer and upload the data
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

    // We now construct a vertex array to describe the format of the input buffer
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

    (vbo, vao)
}

unsafe fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
    let uniform_location = gl.get_uniform_location(program, name);
    // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
    gl.uniform_1_f32(uniform_location.as_ref(), value)
}