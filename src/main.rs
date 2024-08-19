mod window;
use window::Window;

extern crate gl;

fn main() {
    let mut main_window = Window::new(600, 400, "2D Game".into());
    
    while main_window.is_looping() {

        // clear color
        unsafe {
            gl::ClearColor(1.0, 1.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        main_window.update();
    }
}
