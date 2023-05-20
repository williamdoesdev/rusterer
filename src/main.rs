use glow::*;
use std::fs;

mod context;

fn main() {
    unsafe {
        // Create a context from a sdl2 window
        let (gl, window, mut events_loop, _context) = context::create_sdl2_context();
        

        // Create a shader program from source
        let program = create_program(&gl, 
            &fs::read_to_string("res/shaders/triangle.vert.glsl").expect("Should be able to read file"), 
            &fs::read_to_string("res/shaders/triangle.frag.glsl").expect("Should be able to read file"));
        gl.use_program(Some(program));

        // Create a vertex buffer and vertex array object
        let (vbo, vao) = create_vertex_buffer(&gl);
        let ibo = create_index_buffer(&gl);

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // Set color uniform
        let u_color_location = gl.get_uniform_location(program, "uColor");
        gl.uniform_4_f32_slice(u_color_location.as_ref(), &[1.0, 1.0, 0.0, 1.0]);

        let mut green_channel: f32 = 0.0;
        let mut increment: f32 = 0.005;

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

            let u_color_location = gl.get_uniform_location(program, "uColor");
            gl.uniform_4_f32_slice(u_color_location.as_ref(), &[0.25, green_channel, 1.0, 1.0]);

            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            // gl.draw_arrays(glow::TRIANGLES, 0, 6);
            window.gl_swap_window();
        }

        // Clean up
        gl.delete_program(program);
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_buffer(ibo);
    }
}

unsafe fn create_program(
    gl: &glow::Context,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> NativeProgram {
    let program = gl.create_program().expect("Cannot create program");

    let shader_sources = [
        (glow::VERTEX_SHADER, vertex_shader_source),
        (glow::FRAGMENT_SHADER, fragment_shader_source),
    ];

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
        shaders.push(shader);
    }

    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }

    for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }

    return program
}

unsafe fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {

    // This is a flat array of f32s that are to be interpreted as vec2s.
    let square_vertices = [
        -0.5f32, -0.5f32, 
        -0.5f32, 0.5f32, 
        0.5f32, -0.5f32,
        0.5f32, 0.5f32
        ];
    let square_vertices_u8: &[u8] = core::slice::from_raw_parts(
        square_vertices.as_ptr() as *const u8,
        square_vertices.len() * core::mem::size_of::<f32>()
    );

    // We construct a buffer and upload the data
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, square_vertices_u8, glow::STATIC_DRAW);

    // We now construct a vertex array to describe the format of the input buffer
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

    return (vbo, vao)
}

unsafe fn create_index_buffer(gl: &glow::Context) -> NativeBuffer {

    let square_indeces = [
        0, 1, 2,
        1, 2, 3
    ];

    let square_indeces_u8: &[u8] = core::slice::from_raw_parts(
        square_indeces.as_ptr() as *const u8,
        square_indeces.len() * core::mem::size_of::<f32>()
    );

    let ibo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
    gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, square_indeces_u8, glow::STATIC_DRAW);

    return ibo
}

unsafe fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
    let uniform_location = gl.get_uniform_location(program, name);
    // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
    return gl.uniform_1_f32(uniform_location.as_ref(), value)
}