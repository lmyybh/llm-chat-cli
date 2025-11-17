// src/ui/mod.rs
use ratatui::{Frame};

use crate::app::App;

mod layout;
mod input;
mod chat_view;

pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    layout::draw_layout(frame, app, area);
}