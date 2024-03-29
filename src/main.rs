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
mod texture;
use texture::*;

mod boid;
use boid::*;

use glam::*;

fn main() {
    unsafe {

    let (gl, window, mut events_loop, _context) = create_sdl2_context();
        
    let shaders = vec![
    ("res/shaders/texture.vert.glsl", glow::VERTEX_SHADER),
    ("res/shaders/texture.frag.glsl", glow::FRAGMENT_SHADER),
    ];

    let texture = Texture2D::new(&gl, "res/agent.png");
    texture.bind(0);

    let program = NativeProgram::new_from_files(&gl, shaders);
    program.bind(&gl);

    let vertices = [
        -0.05f32, -0.12f32, 0.0, 0.0,
        0.05f32, -0.12f32, 1.0, 0.0,
        0.05f32, 0.12f32, 1.0, 1.0, 
        -0.05f32, 0.12f32, 0.0, 1.0
        ];
    let vertex_buffer = VertexBuffer::new(&gl, &vertices);

    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];
    let index_buffer = IndexBuffer::new(&gl, &indices);

    let mut layout = Vec::<VertexAttribute>::new();
    layout.push_attribute::<f32>(2).push_attribute::<f32>(2);

    let vertex_array = NativeVertexArray::new(&gl, &layout);
    vertex_array.bind(&gl);

    let uniform = Uniform::new(&gl, program, "uColor");
    uniform.set([0.25, 1.0, 1.0, 1.0]);

    let slot_uniform = Uniform::new(&gl, program, "uTextureSlot");
    slot_uniform.set(0);

    let proj = Mat4::orthographic_rh_gl(-2.0, 2.0, -1.5, 1.5, -1.0, 1.0);
    let view = Mat4::from_translation(Vec3::new(-0.0, 0.0, 0.0));
    let mvp_uniform = Uniform::new(&gl, program, "uMVPMatrix");

    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    let renderer = Renderer::new(&gl);

    

    let total_boids = 10;
    let mut boids = Vec::<Boid>::new();

    for i in 0..total_boids {
        let random_pos = Vec3::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0, 0.0);
        let random_rot = Mat4::from_rotation_z(rand::random::<f32>() * 360.0f32.to_radians());

        boids.push(Boid{id: i, position: random_pos, rotation: random_rot, speed: 0.025});
    }

    'render: loop {
        {
            for event in events_loop.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'render;
                }
            }
        }

        renderer.clear();

        for boid in &mut boids {
            mvp_uniform.set(proj * view * Mat4::from_translation(boid.position) * boid.rotation);
            renderer.draw(&vertex_buffer, &vertex_array, &index_buffer, &program);
            boid.next_transform();
        }

        window.gl_swap_window();
    }

    println!("glError: {}", gl.get_error());

    gl.delete_program(program);
    gl.delete_vertex_array(vertex_array);
    }
}