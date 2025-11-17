// src/ui/mod.rs
use ratatui::{Frame};

use crate::app::App;

mod layout;
mod input;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    layout::draw_layout(frame, app, area);
}