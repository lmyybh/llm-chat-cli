// src/app.rs
use unicode_segmentation::UnicodeSegmentation;

pub struct App {
    pub input_buffer: String,
    pub input_cursor: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            input_cursor: 0,
        }
    }

    // 支持中文字符的插入
    pub fn insert_char(&mut self, ch: char) {
        let graphemes: Vec<&str> = self.input_buffer.graphemes(true).collect();
        let before: String = graphemes.iter().take(self.input_cursor).copied().collect();
        let after: String = graphemes.iter().skip(self.input_cursor).copied().collect();
        self.input_buffer = format!("{}{}{}", before, ch, after);
        self.input_cursor += 1;
    }

    // 支持中文字符的删除
    pub fn delete_left(&mut self) {
        if self.input_cursor > 0 {
            let graphemes: Vec<&str> = self.input_buffer.graphemes(true).collect();
            let before: String = graphemes.iter().take(self.input_cursor - 1).copied().collect();
            let after: String = graphemes.iter().skip(self.input_cursor).copied().collect();
            self.input_buffer = format!("{}{}", before, after);
            self.input_cursor -= 1;
        }
    }

    // 处理按键
    pub fn handle_input_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode::*;

        match key.code {
            Enter => {
                self.input_buffer.clear();
                self.input_cursor = 0;
            }
            Char(c) => {
                self.insert_char(c);
            }
            Backspace => {
                self.delete_left();
            }
            Left => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                }
            }
            Right => {
                let len = self.input_buffer.graphemes(true).count();
                if self.input_cursor < len {
                    self.input_cursor += 1;
                }
            }
            _ => {}
        }
    }
}