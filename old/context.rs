use glow::*;
use log::info;

pub unsafe fn create_sdl2_context(window_title: &str) -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    info!("Initializing sdl2...");
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_flags().forward_compatible().set();
    info!("Creating window...");
    let window = video
        .window(window_title, 1024, 769)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    info!("Initializing OpenGL context...");
    let gl_context = window.gl_create_context().unwrap();
    let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
    let event_loop = sdl.event_pump().unwrap();

    info!("OpenGL version: {}.{}", gl.version().major, gl.version().minor);
    info!("Vendor info: {}", gl.version().vendor_info);

    return (gl, window, event_loop, gl_context)
}