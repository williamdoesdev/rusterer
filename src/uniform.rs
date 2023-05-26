use glow::*;

pub struct NativeUniform<'a> {
    gl: &'a glow::Context,
    location: Option<glow::UniformLocation>,
}

impl<'a> NativeUniform<'a> {
    pub fn new(gl: &'a glow::Context, program: glow::Program, name: &str) -> Self {
        unsafe {
            let location = gl.get_uniform_location(program, name);
            return Self {
                gl,
                location,
            }
        }
    }

    pub fn set<T: Uniform>(&self, value: T) {
        value.set_uniform(self.gl, self.location.as_ref());
    }
}

pub trait Uniform {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>);
}

impl Uniform for f32 {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>) {
        unsafe {
            gl.uniform_1_f32(location, self);
        }
    }
}

impl Uniform for [f32; 4] {
    fn set_uniform(self, gl: &glow::Context, location: Option<&glow::UniformLocation>) {
        unsafe {
            gl.uniform_4_f32_slice(location, &self);
        }
    }
}
