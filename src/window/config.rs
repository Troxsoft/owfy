use crate::{
    color::Color,
    math::{Position, Size},
};

use super::window_draw::WindowDraw;

#[derive(Debug, Clone)]
pub struct WinConfig {
    pub on_update: fn(WindowDraw) -> WindowDraw,
    pub on_start: fn(WindowDraw) -> WindowDraw,
    pub title: String,
    pub size: Size<u32>,
    pub position: Position<i32>,
    pub backcolor: Color,
}

impl Default for WinConfig {
    fn default() -> Self {
        Self {
            on_update: |draw| draw,
            on_start: |draw| draw,
            title: "owfy window".to_string(),
            position: Position { x: 200, y: 200 },
            size: Size { w: 400, h: 300 },
            backcolor: Color::blue(),
        }
    }
}
impl WinConfig {
    pub fn size(&self, size: Size<u32>) -> Self {
        return Self {
            on_start: self.on_start,
            on_update: self.on_update,
            title: self.title.clone(),
            position: self.position,
            size: size.clone(),
            backcolor: self.backcolor.clone(),
        };
    }
    pub fn back_color(&self, color: Color) -> Self {
        return Self {
            on_start: self.on_start,
            on_update: self.on_update,
            title: self.title.clone(),
            position: self.position,
            size: self.size.clone(),
            backcolor: color.clone(),
        };
    }
    pub fn position(&self, position: Position<i32>) -> Self {
        return Self {
            on_start: self.on_start,
            on_update: self.on_update,
            title: self.title.clone(),
            position: position.clone(),
            size: self.size.clone(),
            backcolor: self.backcolor.clone(),
        };
    }
    pub fn start(&self, start: fn(WindowDraw) -> WindowDraw) -> Self {
        return Self {
            on_start: start,
            on_update: self.on_update,
            title: self.title.clone(),
            position: self.position,
            size: self.size,
            backcolor: self.backcolor.clone(),
        };
    }
    pub fn update(&self, update: fn(WindowDraw) -> WindowDraw) -> Self {
        return Self {
            on_start: self.on_start,
            on_update: update,
            title: self.title.clone(),
            position: self.position,
            size: self.size,
            backcolor: self.backcolor.clone(),
        };
    }
    pub fn title(&self, title: impl ToString) -> Self {
        return Self {
            on_start: self.on_start,
            on_update: self.on_update,
            title: title.to_string(),
            position: self.position,
            size: self.size,
            backcolor: self.backcolor.clone(),
        };
    }
}
