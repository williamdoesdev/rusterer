use glow::HasContext;
use image::GenericImageView;

pub struct Texture2D<'a> {
    gl: &'a glow::Context,
    texture_id: glow::NativeTexture,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
}

impl<'a> Texture2D<'a> {
    pub fn new(gl: &'a glow::Context, path: &str) -> Self {
        let img = image::open(path).unwrap().flipv();
        let (width, height) = img.dimensions();
        let bpp = img.color().bits_per_pixel() as u32;
        let img = img.to_rgba8();

        unsafe {
            let texture = gl.create_texture().expect("Could not create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&img.into_raw()),
            );
            
            Texture2D { gl: gl, texture_id: texture, width, height, bpp }
        }
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + slot); // The enum values for TEXTURE0, TEXTURE1, etc. are all consecutive, so you can just add the slot value like this
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture_id));
        }
    }
}

impl<'a> Drop for Texture2D<'a> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture_id);
        }
    }
}