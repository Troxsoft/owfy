extern crate gl;
use gl::types::*;
extern crate glfw;
use glfw::Context;
pub mod config;
pub mod window_draw;
use crate::{
    color::Color,
    math::{Position, Size},
};

use self::{config::WinConfig, window_draw::WindowDraw};
#[derive(Debug)]
pub struct Window {
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub glfw_instance: glfw::Glfw,
    position: Position<i32>,
    size: Size<u32>,
    title: String,
    backcolor: Color,
    init_config: WinConfig,
}
impl Window {
    pub fn new(config: WinConfig) -> Self {
        let mut confi2 = config.clone();
        let mut glw = glfw::init(error_message).unwrap();
        glw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glw.window_hint(glfw::WindowHint::Resizable(false));
        let mut res = glw
            .create_window(
                config.size.w,
                config.size.h,
                &config.title.to_string(),
                glfw::WindowMode::Windowed,
            )
            .expect("fail in window create");
        res.0.set_pos(config.position.x, config.position.y);
        let mut pos: Position<i32> = Position::new(res.0.get_pos().0, res.0.get_pos().1);
        return Self {
            events: res.1,
            glfw_instance: glw,
            window: res.0,
            position: pos,
            size: confi2.size,
            title: confi2.title,
            backcolor: confi2.backcolor.clone(),
            init_config: config,
        };
    }
    pub fn init_window(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);

        // loading a specific function pointer
        gl::load_with(|ptr| self.window.get_proc_address(ptr) as *const _);
        let (screen_width, screen_height) = self.window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);

            gl::ClearColor(
                self.backcolor.r,
                self.backcolor.g,
                self.backcolor.b,
                self.backcolor.a,
            );
        }
        let mut dr = WindowDraw::new(self.glfw_instance.clone());
        dr = (self.init_config.on_start)(dr);

        while !self.window.should_close() {
            self.glfw_instance.wait_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                Self::handle_window_event(&mut self.window, event);
            }

            unsafe {
                gl::ClearColor(
                    self.backcolor.r,
                    self.backcolor.g,
                    self.backcolor.b,
                    self.backcolor.a,
                );
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            dr = (self.init_config.on_update)(dr);
            self.window.swap_buffers();
        }
        //vbos
        for val in dr.vbos.values() {
            unsafe {
                gl::DeleteBuffers(1, val);
            }
        }
        for val in dr.programs.values() {
            unsafe {
                gl::DeleteProgram(*val);
            }
        }
        for val in dr.vaos.values() {
            unsafe {
                gl::DeleteVertexArrays(1, val);
            }
        }
    }

    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
    pub fn set_back_color(&mut self, new_color: Color) {
        self.backcolor = new_color;
    }
}
fn error_message(err: glfw::Error, description: String) {
    println!("GLFW error {:?}: {:?}", err, description);
}
