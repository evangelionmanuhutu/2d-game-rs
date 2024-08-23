extern crate glfw;
extern crate gl;

use std::borrow::BorrowMut;

use glfw::{Action, Context, Glfw, GlfwReceiver, Key, MouseButton, PWindow, WindowEvent};

pub struct Window {
    width: u32,
    height: u32,
    glfw_context: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>
}

impl Window{
    pub fn init(width: u32, height: u32, mut title: &str) -> Self {
        use glfw::fail_on_errors;
        let mut glfw_context = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw_context
            .create_window(width, height, &mut title, glfw::WindowMode::Windowed)
            .expect("Failed to create window");
        
        window.set_key_polling(true);
        window.set_size_polling(true);

        // inti opengl
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        Self{
            width,
            height,
            glfw_context,
            window,
            events
        }
    }

    pub fn is_looping(&self) -> bool {
        !self.window.should_close()
    }

    pub fn update(&mut self) {
        self.handle_window_event();
        self.glfw_context.poll_events();
        self.window.swap_buffers();
    }

    fn handle_window_event(&mut self) {
        let events: Vec<WindowEvent> = glfw::flush_messages(&self.events)
            .map(|(_, event)|event).collect();

        for event in events {
            match event {
                WindowEvent::Size(width, height) => {
                    self.handle_window_resize(width, height);
                },
                _ => {},
            }
        }
    }

    fn handle_window_resize(&mut self, width: i32, height: i32) {
        self.width = width as u32;
        self.height = height as u32;
        unsafe { gl::Viewport(0, 0, width, height) };
        println!("Window resized to {}x{}", self.width, self.height);
    }

    pub fn get_window_handle(&mut self) -> &mut glfw::PWindow {
        self.window.borrow_mut()
    }

    pub fn get_glfw_context(&self) -> &glfw::Glfw {
        &self.glfw_context
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        let action = self.window.get_key(key);
        action == Action::Press
    }

    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        let action = self.window.get_mouse_button(mouse_button);
        action == Action::Press
    }

    pub fn get_cursor_pos(&self) -> (f64, f64) {
        let (x, y) = self.window.get_cursor_pos();
        (x, y)
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true)
    }

}