use log::*;
use glow::*;
use std::fs;
use std::mem;

pub struct ShaderProgram<'a> {
    gl: &'a glow::Context,
    program: glow::NativeProgram,
    shaders: Vec::<NativeShader>
}

impl<'a> ShaderProgram<'a> {
    pub unsafe fn new(gl: &'a glow::Context) -> Self{
        return ShaderProgram {
            gl: gl,
            program: gl.create_program().expect("Cannot create program"),
            shaders: vec![]
        }
    }

    pub unsafe fn bind(&mut self) {
        self.gl.link_program(self.program);
        if !self.gl.get_program_link_status(self.program) {
            panic!("{}", self.gl.get_program_info_log(self.program));
        }
        self.gl.use_program(Some(self.program));
    }

    pub unsafe fn unbind(&mut self) {
        // self.gl.program;
    }

    pub unsafe fn set_uniform(&mut self, name: &str, value: &[f32]) {
        let u_color_location = self.gl.get_uniform_location(self.program, name);
        self.gl.uniform_4_f32_slice(u_color_location.as_ref(), value);
    }

    pub unsafe fn compile_shader(&mut self, path: &str, shader_type: u32) {
        let shader_source = &fs::read_to_string(path).expect("Should be able to read file");

        let shader = self.gl
            .create_shader(shader_type)
            .expect("Cannot create shader");
        self.gl.shader_source(shader, shader_source);
        self.gl.compile_shader(shader);
        if !self.gl.get_shader_compile_status(shader) {
            panic!("{}", self.gl.get_shader_info_log(shader));
        }
        self.gl.attach_shader(self.program, shader);

        self.shaders.push(shader);
    }
}


// TODO: This sucks, really I need to just redo this whole project T_T
impl<'a> Drop for ShaderProgram<'a> {
    fn drop(&mut self) {
        info!("Dropping shader...");

        let shaders = mem::take(&mut self.shaders);

        unsafe {
            for shader in shaders {
                self.gl.detach_shader(self.program, shader);
                self.gl.delete_shader(shader);
            }
            self.gl.delete_program(self.program);
        }
    }
}