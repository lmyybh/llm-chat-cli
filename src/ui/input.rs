use ratatui::{
    Frame, layout::Rect, text::Line, widgets::{Block, Borders, Paragraph}
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::app::App;

pub fn render_input(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Input (Press Ctrl+C to quit)");

    // 输入框的文本内容
    let content = Line::raw(&app.input_buffer);

    let paragraph = Paragraph::new(content).block(block);

    frame.render_widget(paragraph, area);

    // 显示光标
    if let Some(cursor_pos) = calculate_cursor_position(app, area) {
        frame.set_cursor_position(cursor_pos);
    }
}

fn calculate_cursor_position(app: &App, area: Rect) -> Option<(u16, u16)> {
    let graphemes: Vec<&str> = app.input_buffer.graphemes(true).collect();
    let text_before_cursor: String = graphemes.iter().take(app.input_cursor).copied().collect();
    let width = UnicodeWidthStr::width(text_before_cursor.as_str());

    let cursor_x = area.x + 1 + width as u16;

    if cursor_x < area.x + area.width {
        Some((cursor_x, area.y + 1))
    } else {
        None
    }
}