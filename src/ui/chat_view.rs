use ratatui::{
    Frame, layout::{Position, Rect}, style::{Color, Style}, text::{Line, Text}, widgets::{Block, Borders, Clear, Paragraph, Wrap}
};
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;

use crate::{app::App, model::{role::Role, message::Message}};

const USER_COLOR: Color = Color::Green;
const ASSISTANT_COLOR: Color = Color::Blue;
const MAX_BUBBLE_WIDTH_PRECENT: u16 = 30;
const BUBBLE_MARGIN: u16 = 1;
const BUBBLE_SPACING: u16 = 1;

#[derive(Debug, Clone)]
struct RenderedMessage {
    role: Role,
    timestamp_str: String,
    content_lines: Vec<String>,
    info_height: u16,
    bubble_width: u16,
    bubble_height: u16,
    color: Color,
}

impl RenderedMessage {
    fn from_message(msg: &Message, max_content_width: usize) -> Self {
        let wrapped_lines = wrap_text(&msg.content, max_content_width);
        let content_height = wrapped_lines.len() as u16;
        let content_width = wrapped_lines
            .iter()
            .map(|line| UnicodeWidthStr::width(line.as_str()) as u16)
            .max()
            .unwrap_or(1);

        let width = (content_width + BUBBLE_MARGIN * 2).min(max_content_width as u16 + BUBBLE_MARGIN * 2);
        let height = content_height + BUBBLE_MARGIN * 2;
        let color = if msg.role == Role::User { USER_COLOR } else { ASSISTANT_COLOR };

        Self {
            role: msg.role.clone(),
            timestamp_str: msg.timestamp.format("%H:%M:%S").to_string(),
            content_lines: wrapped_lines,
            info_height: 1,
            bubble_width: width,
            bubble_height: height,
            color,
        }
    }
}

pub fn render_chat_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Chat");
    frame.render_widget(block.clone(), area);

    let mut inner_area = block.inner(area);
    if inner_area.height == 0 || inner_area.width == 0 {
        return;
    }

    let conversation = app.current_conversation();
    if conversation.messages.is_empty() {
        return;
    }

    // 计算气泡最大宽度
    let max_bubble_width = (inner_area.width as f32 * (MAX_BUBBLE_WIDTH_PRECENT as f32 / 100.0)) as u16;
    let max_bubble_width = max_bubble_width.max(20); // 至少 20 列
    // 气泡内文本最大宽度
    let max_content_width = (max_bubble_width - BUBBLE_MARGIN * 2) as usize;

    let rendered_msgs: Vec<RenderedMessage> = conversation
        .messages
        .iter()
        .map(|m| RenderedMessage::from_message(m, max_content_width))
        .collect();

    let total_height: u16 = rendered_msgs
        .iter()
        .map(|m| m.info_height + m.bubble_height + BUBBLE_SPACING)
        .sum::<u16>()
        .saturating_sub(BUBBLE_SPACING); // 最后一个气泡没有间距

    let needs_scrollbar = total_height > inner_area.height;
    if needs_scrollbar {
        inner_area.width = inner_area.width.saturating_sub(1);
    }

    // 计算对话内容从哪里开始可见
    let visible_top: u16 = if total_height <= inner_area.height {
        app.chat_scroll_offset = 0; // 不足一屏时，默认置底
        0
    } else {
        app.chat_scroll_offset = app.chat_scroll_offset.min(total_height - inner_area.height);
        total_height.saturating_sub(inner_area.height).saturating_sub(app.chat_scroll_offset)
    };

    let mut current_y = 0u16; // 对话内容的 y
    let mut draw_y = inner_area.y; // 绘制区域的 y

    for msg in &rendered_msgs {
        let msg_full_height = msg.info_height + msg.bubble_height + BUBBLE_SPACING;

        let msg_bottom = current_y + msg_full_height;
        // 消息在可见区域上方，跳过
        if msg_bottom <= visible_top {
            current_y += msg_full_height;
            continue;
        }

        // 消息在可见区域下方，停止绘制
        if current_y >= visible_top + inner_area.height {
            break;
        }

        // 当前消息要隐藏的顶部行数
        let clip_top = if current_y < visible_top { visible_top - current_y } else { 0 };

        let available_draw_height = inner_area.height.saturating_sub(draw_y) + 1;
        let draw_height = (msg.info_height + msg.bubble_height as u16).saturating_sub(clip_top).min(available_draw_height);

        if draw_height == 0 {
            current_y += msg_full_height;
            continue;
        }

        let info_draw_height = msg.info_height.saturating_sub(clip_top);
        let bubble_draw_height = draw_height.saturating_sub(info_draw_height);
        let bubble_clip_top = clip_top.saturating_sub(msg.info_height);

        if info_draw_height > 0 {
            let info_rect = Rect {x: inner_area.x + 1, y: draw_y, width: 20, height: info_draw_height};
            frame.render_widget(Clear, info_rect);
            let line = Line::from(format!("{} {}", msg.role.to_string(), msg.timestamp_str))
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(Paragraph::new(Text::from(vec![line])), info_rect);

            draw_y += info_draw_height;
        }

        if bubble_draw_height > 0 {
            let bubble_rect = Rect {
                x: inner_area.x + 1,
                y: draw_y,
                width: msg.bubble_width,
                height: bubble_draw_height,
            };

            frame.render_widget(Clear, bubble_rect);

            // 绘制气泡边框
            draw_bubble_borders(frame, &bubble_rect, bubble_clip_top, msg.bubble_height, msg.color);

            // 绘制气泡内容
            if bubble_draw_height >= 2 {
                let border_height = (BUBBLE_MARGIN * 2).saturating_sub(msg.bubble_height.saturating_sub(bubble_draw_height)).min(BUBBLE_MARGIN * 2).max(0);
                let content_visible_height = bubble_draw_height - border_height; // 预留上下边框空间
                let content_start_line = bubble_clip_top.saturating_sub(1); // 跳过被裁剪的“上边框+若干行”
                let start_idx = content_start_line.min(msg.content_lines.len() as u16) as usize;

                let content_lines_to_show = msg
                    .content_lines
                    .iter()
                    .skip(start_idx)
                    .take(content_visible_height as usize)
                    .cloned()
                    .collect::<Vec<_>>();

                if !content_lines_to_show.is_empty() {
                    let content_rect = Rect {
                        x: bubble_rect.x + 1,
                        y: if bubble_clip_top == 0 { bubble_rect.y + 1 } else { bubble_rect.y },
                        width: bubble_rect.width.saturating_sub(2),
                        height: if bubble_clip_top == 0 && bubble_draw_height == msg.bubble_height {
                            content_visible_height
                        } else {
                            bubble_draw_height - if bubble_clip_top == 0 { 1 } else { 0 }
                        },
                    };

                    if content_rect.width > 0 && content_rect.height > 0 {
                        let text = Text::from(
                            content_lines_to_show
                                .into_iter()
                                .map(Line::from)
                                .collect::<Vec<Line>>(),
                        );
                        let para = Paragraph::new(text)
                            .style(Style::default().fg(msg.color))
                            .wrap(Wrap { trim: true });
                        frame.render_widget(para, content_rect);
                    }
                }
            }

            draw_y += bubble_draw_height;
        }

        current_y += msg_full_height;
        draw_y += BUBBLE_SPACING;
    }

    let scrollbar_area = block.inner(area); // 包含完整宽度
    draw_scrollbar(
        frame, 
        scrollbar_area, 
        total_height, 
        inner_area.height,
        total_height.saturating_sub(inner_area.height).saturating_sub(app.chat_scroll_offset) // chat_scroll_offset 表示的是倒序 offset
    );
}

fn draw_bubble_borders(frame: &mut Frame, rect: &Rect, clip_top: u16, original_height: u16, color: Color) {
    if rect.height == 0 || rect.width == 0 {
        return;
    }

    let buf = frame.buffer_mut();
    let style = Style::default().fg(color);

    // 左右边框：始终画满高度
    for y in rect.top()..rect.bottom() {
        if y < buf.area.bottom() {
            buf[Position {x: rect.left(), y}].set_symbol("│").set_style(style);
            if rect.width > 1 {
                buf[Position {x: rect.right() - 1, y}].set_symbol("│").set_style(style);
            }
        }
    }

    // 上边框：仅当未被裁剪（clip_top == 0）
    if clip_top == 0 && rect.height > 0 {
        for x in rect.left() + 1..rect.right() - 1 {
            if x < buf.area.right() && rect.top() < buf.area.bottom() {
                buf[Position {x, y: rect.top()}].set_symbol("─").set_style(style);
            }
        }
        // 角落
        if rect.width > 1 {
            buf[Position {x: rect.left(), y: rect.top()}].set_symbol("┌").set_style(style);
            buf[Position {x: rect.right() - 1, y: rect.top()}].set_symbol("┐").set_style(style);
        } else {
            buf[Position {x: rect.left(), y: rect.top()}].set_symbol("╷").set_style(style);
        }
    }

    // 下边框：仅当完整显示底部（即未被底部裁剪）
    let is_bottom_visible = clip_top + rect.height == original_height;
    if is_bottom_visible && rect.height > (if clip_top == 0 { 1 } else { 0 }) {
        let bottom_y = rect.bottom() - 1;
        if bottom_y < buf.area.bottom() {
            for x in rect.left() + 1..rect.right() - 1 {
                if x < buf.area.right() {
                    buf[Position {x, y: bottom_y}].set_symbol("─").set_style(style);
                }
            }
            if rect.width > 1 {
                buf[Position {x: rect.left(), y: bottom_y}].set_symbol("└").set_style(style);
                buf[Position {x: rect.right() - 1, y: bottom_y}].set_symbol("┘").set_style(style);
            } else {
                buf[Position {x: rect.left(), y: bottom_y}].set_symbol("╵").set_style(style);
            }
        }
    }
}

fn draw_scrollbar(frame: &mut Frame, area: Rect, total_height: u16, visible_height: u16, scroll_offset: u16) {
    if total_height <= visible_height || visible_height == 0 {
        return;
    }

    let scrollbar_width = 1;
    let scrollbar_x = area.right().saturating_sub(scrollbar_width);
    if scrollbar_x <= area.left() {
        return;
    }

    let track_height = visible_height;
    let thumb_height = ((visible_height as f64 / total_height as f64) * track_height as f64).max(1.0) as u16;
    let max_scroll = total_height.saturating_sub(visible_height);
    let scroll_ratio = scroll_offset as f64 / max_scroll as f64;
    let thumb_y_offset = (scroll_ratio * (track_height - thumb_height) as f64).round() as u16;
    let thumb_top = area.top() + thumb_y_offset;

    let buf = frame.buffer_mut();
    // let style_track = Style::default().fg(Color::DarkGray);
    let style_thumb = Style::default().fg(Color::Gray);

    for y in thumb_top..thumb_top.saturating_add(thumb_height) {
        if y >= area.bottom() {
            break;
        }
        // 使用实心块或竖线
        buf[Position {x: scrollbar_x, y}]
            .set_symbol("▐") // 或 "█", "│"
            .set_style(style_thumb);
    }
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return vec!["".to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in words {
        let word_width = UnicodeWidthStr::width(word);
        let space_width = if current_line.is_empty() { 0 } else { 1 };
        let needed_width = current_width + space_width + word_width;

        if needed_width <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += 1;
            }
            current_line.push_str(word);
            current_width += word_width;
        } else {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
            }
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // 处理超长单词（如长 URL 或无空格中文）
    let mut final_lines = Vec::new();
    for line in lines {
        if UnicodeWidthStr::width(line.as_str()) > max_width {
            final_lines.extend(break_long_line(line, max_width));
        } else {
            final_lines.push(line);
        }
    }

    final_lines
}

fn break_long_line(s: String, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_w = 0;

    for g in s.graphemes(true) {
        let gw = UnicodeWidthStr::width(g);
        if current_w + gw > max_width && !current.is_empty() {
            lines.push(current.clone());
            current.clear();
            current_w = 0;
        }
        current.push_str(g);
        current_w += gw;
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}