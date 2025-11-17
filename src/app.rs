// src/app.rs
use unicode_segmentation::UnicodeSegmentation;
use crate::model::{
    role::Role,
    message::Message,
    conversation::Conversation,
};

pub struct App {
    pub input_buffer: String,
    pub input_cursor: usize,
    pub conversations: Vec<Conversation>,
    pub current_conversation_index: usize,
    pub chat_scroll_offset: u16,
}

impl App {
    pub fn new() -> Self {
        let mut conv = Conversation::new();
        conv.add_message(Message::new(Role::Assistant, "你好，我是 LLM".to_string()));
        Self {
            input_buffer: String::new(),
            input_cursor: 0,
            conversations: vec![conv],
            current_conversation_index: 0,
            chat_scroll_offset: 0,
        }
    }

    pub fn current_conversation(&self) -> &Conversation {
        &self.conversations[self.current_conversation_index]
    }

    pub fn add_message_to_current_conversation(&mut self, message: Message) {
        self.conversations[self.current_conversation_index].add_message(message);
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
                if !self.input_buffer.trim().is_empty() {
                    let user_msg = Message::new(Role::User, self.input_buffer.clone());
                    self.add_message_to_current_conversation(user_msg);

                    // 模拟回复
                    self.add_message_to_current_conversation(
                        Message::new(Role::Assistant, "这是模拟的 LLM 回复。你可以继续输入！".to_string())
                    );

                    self.input_buffer.clear();
                    self.input_cursor = 0;
                    self.chat_scroll_offset = 0; // 发送新消息时，自动滚动到底部
                }
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