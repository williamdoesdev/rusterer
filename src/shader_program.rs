use glow::*;
use std::fs;

pub trait CreateFromFile {
    fn new_from_files(gl: &glow::Context, shaders: Vec<(&str, u32)>) -> Self;
}

impl CreateFromFile for NativeProgram {
    fn new_from_files(gl: &glow::Context, shaders: Vec<(&str, u32)>) -> Self {
        unsafe {
            // Create new NativeProgram
            let program = gl.create_program().expect("Cannot create program");

            // Create vector to hold shaders which I will delete after I'm done
            let mut shaders_to_delete: Vec<NativeShader> = Vec::with_capacity(shaders.len());

            // Read and compile shader files
            for element in shaders {
                let (path, shader_type) = element;
                let source = fs::read_to_string(path).expect("Cannot read file");
                let shader = gl.create_shader(shader_type).expect("Cannot create shader");
                gl.shader_source(shader, &source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }

                // Attach shader
                gl.attach_shader(program, shader);

                shaders_to_delete.push(shader);
            }
            
            // Link all shaders currently attached to the program.
            gl.link_program(program);   

            // Cleanup
            for shader in shaders_to_delete {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }         

            return program
        }
    }
}

pub trait BindShader {
    fn bind(&self, gl: &glow::Context);
}

impl BindShader for NativeProgram {
    fn bind(&self, gl: &glow::Context) {
        unsafe{
            gl.use_program(Some(*self));
        }
    }
}