use std::{fs::File, io::Read};

extern crate gl;
extern crate stb_image;
use gl::types::*;

pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn create(filepath: &str) -> Self {
        unsafe {
            println!("[Texture] Loading texture from {}", filepath);
            let mut id: GLuint = 0;
            let mut contents = Vec::new();
            
            let mut file = File::open(filepath).expect("File not found");
            let _ = file.read_to_end(&mut contents);

            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut bits_per_pixel: i32 = 0;

            stb_image::stb_image::stbi_set_flip_vertically_on_load(1);
            let img = stb_image::stb_image::stbi_load_from_memory(
                    contents.as_mut_ptr(), contents.len() as i32,
                    &mut width,
                    &mut height,
                    &mut bits_per_pixel,
                    0);

            if img.is_null() {
                panic!("[Texture] Failed to load image");
            }

            println!("[Texture] Image loaded width {} height {} bpp {}", width, height, bits_per_pixel);
            
            let (internal_format, data_format) = match bits_per_pixel {
                4 => (gl::RGBA8 as i32, gl::RGBA),
                3 => (gl::RGB8 as i32, gl::RGB),
                _ => {
                    stb_image::stb_image::stbi_image_free(img as *mut _);
                    panic!("[Texture] Unsupported bits per pixel: {}", bits_per_pixel);
                }
            };

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TextureStorage2D(id, 1, internal_format as u32, width, height);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TextureSubImage2D(id, 0, 0, 0, width, height, data_format, gl::UNSIGNED_BYTE, img as *mut _);

            // Free image memory
            stb_image::stb_image::stbi_image_free(img as *mut _);

            // Check for OpenGL errors
            

            println!("[Texture] Texture created with ID {}", id);
            Self { id }
        }
        
    }

    pub fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, [self.id].as_ptr());}
    }

    pub fn bind(&self, slot: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}