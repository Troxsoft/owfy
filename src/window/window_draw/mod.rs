use std::collections::HashMap;

use crate::{
    color::{self, Color},
    math::{Position, Size},
};

use super::Window;
extern crate gl;
use gl::types::*;
extern crate glfw;
use glfw::Context;
#[derive(Debug, Clone)]
pub struct WindowDraw {
    glfw_instance: glfw::Glfw,
    pub vbos: HashMap<String, u32>,
    pub programs: HashMap<String, u32>,
    pub vaos: HashMap<String, u32>,
    pub last_program: u32,
}
impl WindowDraw {
    pub fn new(glfw_instance: glfw::Glfw) -> Self {
        return Self {
            glfw_instance,
            vbos: HashMap::new(),
            programs: HashMap::new(),
            vaos: HashMap::new(),
            last_program: 0,
        };
    }
    pub fn pre_triangle(&mut self, id: impl ToString, color: Color) -> Result<(), String> {
        // 1 mandar a la gpu(targeta grafica)
        unsafe {
            let mut vbo: u32 = 0;
            let vertecies = [-0.5f32, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
            let mut vao: u32 = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertecies) as isize,
                vertecies.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
            //vertex shader
            const VERTEX_SHADER_SOURCE: &str = "#version 330 core
layout (location = 0) in vec3 position;
    
void main()
{
    gl_Position = vec4(position.x, position.y, position.z, 1.0);
}";
            let color2 = color.clone();
            let FRAGMENT_SHADER_SOURCE: &str = &format!(
                "#version 330 core
out vec4 FragColor;
void main()
{{
    FragColor = vec4({}, {}, {}, {});
}}",
                color2.r, color2.g, color2.b, color2.a,
            );

            let mut vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let mut fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &VERTEX_SHADER_SOURCE.as_bytes().as_ptr().cast(),
                &VERTEX_SHADER_SOURCE.len().try_into().unwrap(),
            );
            gl::ShaderSource(
                fragment_shader,
                1,
                &FRAGMENT_SHADER_SOURCE.as_bytes().as_ptr().cast(),
                &FRAGMENT_SHADER_SOURCE.len().try_into().unwrap(),
            );
            gl::CompileShader(vertex_shader);
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            // error fragment
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                return Err(String::from_utf8_lossy(&v).to_string());
            }
            // error vertex
            success = 0;
            // error fragment
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                return Err(String::from_utf8_lossy(&v).to_string());
            }
            // verificar si hay un error en tiempo de ejecuon -Listo
            // programa
            let mut shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            // err
            success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                return Err(String::from_utf8_lossy(&v).to_string());
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            self.vbos.insert(id.to_string(), vbo);
            self.programs.insert(id.to_string(), shader_program);
            self.vaos.insert(id.to_string(), vao);
            Ok(())
        }
    }
    pub fn triangle(&mut self, id: impl ToString) {
        unsafe {
            if self.last_program != *self.programs.get(&id.to_string()).unwrap() {
                gl::UseProgram(*self.programs.get(&id.to_string()).unwrap());
                gl::BindVertexArray(*self.vaos.get(&id.to_string()).unwrap());
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                gl::BindVertexArray(0);
                self.last_program = *self.programs.get(&id.to_string()).unwrap();
            } else {
                gl::BindVertexArray(*self.vaos.get(&id.to_string()).unwrap());
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                gl::BindVertexArray(0);
            }
        }
    }
}
