extern crate nalgebra_glm as glm;
use glm::{Mat4, Vec2, Vec3};

pub struct Camera {
    ortho_scale: f32,
    ortho_size: Vec2,
    aspect_ratio: f32,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    position: Vec3,
    viewport_size: Vec2
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let aspect_ratio = width / height;
        let ortho_scale: f32 = 10.0;
        let ortho_size = Vec2::new(ortho_scale * aspect_ratio / 2.0, ortho_scale / 2.0);

        // Initialize matrices
        let mut camera = Self {
            ortho_scale,
            ortho_size,
            aspect_ratio,
            projection_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            position: Vec3::new(0.0, 0.0, 0.0),
            viewport_size: Vec2::new(width, height),
        };

        // Update matrices during initialization
        camera.update_projection();
        camera.update_view();

        camera
    }

    pub fn update(&mut self, delta_time: f32) {
        self.update_projection();
        self.update_view();
    }

    fn update_projection(&mut self) {
        let near: f32 = 0.1;
        let far: f32 = 100.0;

        self.projection_matrix = glm::ortho(
            -self.ortho_size.x, 
            self.ortho_size.x, 
            -self.ortho_size.y,
            self.ortho_size.y, 
            near, 
            far);
    }

    fn update_view(&mut self) {
         // Create a view matrix that translates the world to the camera's position
        self.view_matrix = glm::translate(&glm::Mat4::identity(), &(-self.position));
    }

    pub fn get_view_projection(&self) -> Mat4 {
        self.projection_matrix * self.view_matrix
    }

    pub fn on_resize(&mut self, width: f32, height: f32) {
        self.aspect_ratio = 16.0 / 9.0;

        if width / height > self.aspect_ratio {
            self.viewport_size.y = height;
            self.viewport_size.x = height * self.aspect_ratio;
        } 
        else {
            self.viewport_size.x = width;
            self.viewport_size.y = height / self.aspect_ratio;
        }

        self.ortho_size.x = self.ortho_scale * self.aspect_ratio / 2.0;
        self.ortho_size.y = self.ortho_scale / 2.0;
    }

}

