use gl::types::*;

use glam::Mat4;

use std::ffi::CString;
use std::ptr;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        unsafe {
            let vertex_shader =
                Shader::compile_shader(vertex_source, gl::VERTEX_SHADER);

            let fragment_shader =
                Shader::compile_shader(fragment_source, gl::FRAGMENT_SHADER);

            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            let mut success = 0;

            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                panic!("Shader linking failed");
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Self { id: program }
        }
    }

    fn compile_shader(source: &str, shader_type: u32) -> u32 {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            let c_str = CString::new(source.as_bytes()).unwrap();

            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());

            gl::CompileShader(shader);

            let mut success = 0;

            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                panic!("Shader compilation failed");
            }

            shader
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, matrix: &Mat4) {
        unsafe {
            let c_name = CString::new(name).unwrap();

            let location =
                gl::GetUniformLocation(self.id, c_name.as_ptr());

            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                matrix.to_cols_array().as_ptr(),
            );
        }
    }
}
