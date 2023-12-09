use ratatui::style::Color;

pub fn level_color(v: f32) -> Color {
    if v > 0.8 {
        Color::Red
    } else if v > 0.6 {
        Color::Yellow
    } else {
        Color::Green
    }
}
