extern crate glfw;
extern crate gl;

use std::mem::*;
use gl::types::*;
use std::os::raw::c_void;
use glfw::{Action, Context, Key};

fn compile_shader(shader_type: GLenum, source: &str) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type); // create a shader
        // let c_str = std::ffi::CString::new(source).unwrap();
        // let c_str_ptr = c_str.as_ptr();
        let source_ptr = source.as_ptr() as *const i8;
        gl::ShaderSource(
            shader, // shader to compile
            1, // number of strings to compile
            &source_ptr, // pointer to the array of pointers, or a pointer to a pointer in this case
            &(source.len() as i32) // length of the strings, null means that the strings are null-terminated
        ); 
        gl::CompileShader(shader); // compile the shader

        // Check for errors
        let mut success: GLint = 1;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let error = std::ffi::CString::new(std::iter::repeat(' ').take(len as usize).collect::<String>()).unwrap();
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            println!("Shader compilation failed: {}", error.to_str().unwrap());
            gl::DeleteShader(shader);
            return 0;
        }

        return shader
    }
}

fn create_shader(vertex_shader: &str, fragment_shader: &str) -> u32 {
    unsafe {
        // Create a program and attach the shaders
        let program = gl::CreateProgram();
        let vs = compile_shader(gl::VERTEX_SHADER, vertex_shader);
        let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_shader);

        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        gl::ValidateProgram(program);

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        return program;
    }
}

fn main() {
    // Initialize the glfw library
    let mut glfw_context = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not initialize glfw");

    println!("GLFW version: {}", glfw::get_version_string());

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw_context.create_window(300, 300, "Rusterer", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);



    // Vertex buffer
    let positions = [
    -0.5, -0.5, 0.0, 1.0,
     0.0,  0.5, 0.0, 1.0,
     0.5, -0.5, 0.0, 1.0,
    ];

    let mut vertex_buffer: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer); // generate a buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer); // bind the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER, // type of buffer
            (positions.len() * size_of::<f32>()) as GLsizeiptr, // size in bytes
            positions.as_ptr() as *const c_void, // data pointer
            gl::STATIC_DRAW // mode
        );

        gl::VertexAttribPointer(
            0, // index
            4, // size (number of components)
            gl::FLOAT, // type
            gl::FALSE, // whether or not to normalize the data
            (size_of::<f32>() * 4) as i32, // stride between vertices
            std::ptr::null() // Offset of the attribute in the vertex
        );
        gl::EnableVertexAttribArray(0);
    }



    // Shader
    let vertex_shader = r#"
        #version 330 core

        layout(location = 0) in vec4 position;

        void main() {
            gl_Position = position;
        }
        "#;

    let fragment_shader = r#"
        #version 330 core

        layout(location = 0) out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#;

    let shader = create_shader(vertex_shader, fragment_shader);
    unsafe {
        gl::UseProgram(shader);
    }
    
    // Loop until the user closes the window
    while !window.should_close() {
        // Set viewport size
        unsafe{
            let (width, height) = window.get_size();
            gl::Viewport(0, 0, width, height);
        }
        // Clear the screen
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        //Render happens here
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw_context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}