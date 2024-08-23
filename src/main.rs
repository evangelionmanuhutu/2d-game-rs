mod renderer;
mod camera;
mod scene;

extern crate nalgebra_glm as glm;
extern crate gl;

use renderer::texture::Texture;
use renderer::window::Window;
use renderer::shader::Shader;

use camera::Camera;
use glfw::Key;

use std::mem::size_of;

fn main() {
    let window_width: u32 = 600;
    let window_height: u32 = 400;
    let mut window = Window::init(window_width, window_height, "2D Game");
    
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;

    let vertices: [f32; 20] = [
         0.5,  0.5, 0.0, 1.0, 1.0,
         0.5, -0.5, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 0.0,
        -0.5,  0.5, 0.0, 0.0, 1.0
    ];

    let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
    unsafe {
        gl::CreateVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::CreateBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * size_of::<f32>()) as isize,  vertices.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::CreateBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<f32>()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * size_of::<f32>() as i32, std::ptr::null());
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * size_of::<f32>() as i32, (3 * size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
    }

    let mut shader = Shader::create("shaders/shader.vert","shaders/shader.frag");
    let mut camera = Camera::new(600.0, 400.0, 5.0);
    let identity = glm::Mat4::identity();
    let model_matrix = glm::translate(&identity, &glm::Vec3::new(0.0, 0.0, -1.0));

    let mut last_frame_time: f64 = 0.0;

    let mut texture = Texture::create("textures/rust.png");

    let speed: f32 = 5.0;
    while window.is_looping() {
        let current_time  = window.get_glfw_context().get_time();
        let delta_time = current_time - last_frame_time;
        last_frame_time = current_time;

        if window.is_key_pressed(Key::A) {
            camera.position.x -= speed * delta_time as f32;
        } else if window.is_key_pressed(Key::D) {
            camera.position.x += speed * delta_time as f32;
        }

        if window.is_key_pressed(Key::W) {
            camera.position.y += speed * delta_time as f32;
        } else if window.is_key_pressed(Key::S) {
            camera.position.y -= speed * delta_time as f32;
        }

        if window.is_key_pressed(Key::Escape) {
            window.close()
        }

        camera.update();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();
            texture.bind(0);
            shader.set_int1("sampler0", 0);
            shader.set_matrix4("viewProjection", camera.get_view_projection());
            shader.set_matrix4("model", model_matrix);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
        
        window.update();
    }

    texture.drop();
    shader.drop();
}

