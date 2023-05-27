use glow::*;

use glam::*;

pub struct Uniform<'a> {
    gl: &'a glow::Context,
    location: Option<glow::UniformLocation>,
}

impl<'a> Uniform<'a> {
    pub fn new(gl: &'a glow::Context, program: glow::Program, name: &str) -> Self {
        unsafe {
            let location = gl.get_uniform_location(program, name);
            return Self {
                gl,
                location,
            }
        }
    }

    pub fn set<T: IsUniform>(&self, value: T) {
        value.set_uniform(self.gl, self.location.as_ref());
    }
}

pub trait IsUniform {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>);
}

impl IsUniform for f32 {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>) {
        unsafe {
            gl.uniform_1_f32(location, self);
        }
    }
}

impl IsUniform for i32 {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>) {
        unsafe {
            gl.uniform_1_i32(location, self);
        }
    }
}

// Arrays

impl IsUniform for [f32; 4] {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>) {
        unsafe {
            gl.uniform_4_f32_slice(location, &self);
        }
    }
}

// Matrices

impl IsUniform for Mat4 {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>) {
        unsafe {
            println!("mat4: {}", self);
            println!("to_cols_array(): ");
            for item in self.to_cols_array() {
                print!("{} ", item)
            }
            println!();
            gl.uniform_matrix_4_f32_slice(location, true, &self.to_cols_array());
        }
    }
}