extern crate glfw;
extern crate gl;

use std::mem::*;
use gl::types::*;
use std::os::raw::c_void;
use glfw::{Action, Context, Key};

fn main() {
    // Initialize the glfw library
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not initialize glfw");

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(300, 300, "Rusterer", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Vertex buffer
    let positions = [
        -0.5, -0.5,
        0.0, -0.5,
        0.5, -0.5,
    ];

    let mut vertex_buffer: u32 = 0;
    unsafe{
        gl::GenBuffers(1, &mut vertex_buffer); // generate a buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer); // bind the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER, // type of buffer
            (positions.len() * size_of::<f32>()) as GLsizeiptr, // size in bytes
            positions.as_ptr() as *const c_void, // data pointer
            gl::STATIC_DRAW // mode
        );
    }

    // Loop until the user closes the window
    while !window.should_close() {
        //***RENDER HERE***//
        unsafe{
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Clear the screen
        unsafe{
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
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