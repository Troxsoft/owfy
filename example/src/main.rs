use owfy::{
    color::Color,
    math::{Position, Size},
    window::{config::WinConfig, window_draw::WindowDraw, Window},
};
fn main() {
    let mut window = Window::new(
        WinConfig::default()
            .title("hola como estas")
            .back_color(Color::green())
            .start(start)
            .update(update),
    );

    window.init_window();
}
fn start(mut pre_draw: WindowDraw) -> WindowDraw {
    pre_draw.pre_triangle("1", Color::blue()).unwrap();
    pre_draw
}
fn update(mut draw: WindowDraw) -> WindowDraw {
    draw.triangle("1");
    draw
}
