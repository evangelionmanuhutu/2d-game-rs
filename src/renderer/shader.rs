use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

extern crate nalgebra_glm as glm;
extern crate gl;

pub struct Shader {
    id: u32,
    uniform_locations: HashMap<String, i32>,
}

impl Shader {
    pub fn create(vertex_path: &str, fragment_path: &str) -> Self{
        let vertex_source = Shader::read_file_to_string(&vertex_path)
            .expect("Failed to read vertex shader");
        let fragment_source = Shader::read_file_to_string(&fragment_path)
            .expect("Failed to read fragment sahder");
        
        let vertex_shader: u32;
        let fragment_shader: u32;
        let shader_program: u32;
        unsafe { 
            vertex_shader = Shader::create_shader(gl::VERTEX_SHADER, &vertex_source);
            fragment_shader = Shader::create_shader(gl::FRAGMENT_SHADER, &fragment_source);
            shader_program = Shader::create_program(vertex_shader, fragment_shader)
        }

        Self{
            id: shader_program,
            uniform_locations: HashMap::new(),
        }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn get_uniform_location(&mut self, name: &str) -> i32 {
        if let Some(&location ) = self.uniform_locations.get(name) {
            return location;
        }

        // convert c_name
        let c_name = CString::new(name).expect("CString::new failed");
        unsafe {
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            self.uniform_locations.insert(name.to_string(), location);
            location
        }
    }

    pub fn set_matrix3(&mut self, name: &str, value: glm::Mat3) {
        let location = self.get_uniform_location(name);
        unsafe { gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr()) }
    }

    pub fn set_matrix4(&mut self, name: &str, value: glm::Mat4) {
        let location = self.get_uniform_location(name);
        unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr()) }
    }

    pub fn set_vector2(&mut self, name: &str, value: glm::Vec2) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform2f(location, value.x, value.y); }
    }

    pub fn set_vector3(&mut self, name: &str, value: glm::Vec3) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform3f(location, value.x, value.y, value.z); }
    }

    pub fn set_vector4(&mut self, name: &str, value: glm::Vec4) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform4f(location, value.x, value.y, value.z, value.w); }
    }

    unsafe fn create_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
        let shader_program: u32 = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success: gl::types::GLint = 1;
        gl::GetShaderiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as gl::types::GLint {
            // if linking failed, get the error log
            let mut info_log = vec![0; 512];

            gl::GetShaderInfoLog(
                shader_program,
                info_log.len() as i32,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar
            );

            println!(
                "[Shader] Linking failed\n '{}'",
                std::str::from_utf8(&info_log).unwrap()
            );

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            gl::DeleteProgram(shader_program);
            return 0;
        }

        shader_program
    }

    unsafe fn create_shader(types: gl::types::GLenum, source: &str) -> u32 {
        let shader: u32 = gl::CreateShader(types);

        // convert the shader source to a C-compatible string
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader,  1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success: gl::types::GLint = 1;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as gl::types::GLint {
            // if compilation failed, get the error log
            let mut info_log = vec![0; 512];

            gl::GetShaderInfoLog(
                shader,
                info_log.len() as i32,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar
            );

            println!(
                "[Shader] Compilation failed\n '{}'",
                std::str::from_utf8(&info_log).unwrap()
            );

            gl::DeleteShader(shader);
            return 0;
        }

        shader
    }

    fn read_file_to_string(file_path: &str) -> io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }


}