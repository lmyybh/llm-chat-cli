// src/ui/layout.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::{app::App, ui::input};

// layout 布局
// 左边为 sidebar
// 右边为 chat view 和 input
pub fn draw_layout(frame: &mut Frame, app: &App, area: Rect) {
    // Split screen into left (sidebar) and right (chat + input)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);

    let sidebar_area = chunks[0];
    let right_area = chunks[1];

    // Split right area into chat (flexible) and input (fixed 3 lines)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),   // Chat area takes at least 5 lines, grows as needed
            Constraint::Length(3), // Input box fixed to 3 lines
        ])
        .split(right_area);

    let chat_area = right_chunks[0];
    let input_area = right_chunks[1];

    // Render each region with a Block for visualization
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Sidebar"),
        sidebar_area,
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Chat View"),
        chat_area,
    );
    input::render_input(frame, app, input_area);
}