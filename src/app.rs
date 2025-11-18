use std::{env, sync::mpsc};
use unicode_segmentation::UnicodeSegmentation;
use crate::{
    model::openai::{Role, Message, Conversation},
    llm::client::stream_completion,
};

pub struct App {
    pub input_buffer: String,
    pub input_cursor: usize,
    pub conversations: Vec<Conversation>,
    pub current_conversation_index: usize,
    pub chat_scroll_offset: u16,
    pub llm_receiver: Option<mpsc::Receiver<String>>,
    pub is_waiting_for_response: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            input_cursor: 0,
            conversations: vec![Conversation::new()],
            current_conversation_index: 0,
            chat_scroll_offset: 0,
            llm_receiver: None,
            is_waiting_for_response: false,
        }
    }

    pub fn current_conversation(&self) -> &Conversation {
        &self.conversations[self.current_conversation_index]
    }

    pub fn add_message_to_current_conversation(&mut self, message: Message) {
        self.conversations[self.current_conversation_index].add_message(message);
    }

    pub fn add_streaming_content(&mut self, string: String) {
        if let Some(last_msg) = self.conversations[self.current_conversation_index].messages.last_mut() {
            if last_msg.role == Role::Assistant {
                last_msg.content.push_str(&string);
            }
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
                if !self.input_buffer.trim().is_empty() {
                    let user_msg = Message::new(Role::User, self.input_buffer.clone());
                    self.add_message_to_current_conversation(user_msg);

                    // 添加空的 Assistant 消息占位
                    self.add_message_to_current_conversation(
                        Message::new(Role::Assistant, "".to_string())
                    );

                    self.input_buffer.clear();
                    self.input_cursor = 0;

                    self.is_waiting_for_response = true;

                    // 创建 channel
                    let (sender, receiver) = mpsc::channel();
                    self.llm_receiver = Some(receiver);

                    // 流式请求
                    let api_url = env::var("LLM_API_URL")
                        .unwrap_or_else(|_| "http://localhost:8000/v1/chat/completions".to_string());
                    let api_key = env::var("LLM_API_KEY").ok();
                    let model = env::var("LLM_MODEL").unwrap_or_else(|_| "Qwen3/Qwen3-8B".to_string());

                    stream_completion(
                        api_url,
                        api_key,
                        model,
                        self.current_conversation().messages.clone(),
                        sender,
                    );

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

    // 处理流式输出
    pub fn handle_streaming_response(&mut self) {
        if self.llm_receiver.is_none() {
            return;
        }

        let receiver = self.llm_receiver.take().unwrap();

        let mut should_put_back = true;
            let mut done = false;

            // 非阻塞地读取所有可用 token
            while let Ok(token) = receiver.try_recv() {
                if token == "__DONE__" {
                    done = true;
                    should_put_back = false;
                    break;
                } else if token == "__ERROR__" {
                    // 追加错误消息
                    done = true;
                    should_put_back = false;
                    break;
                } else {
                    // 追加 token 到最新 Assistant 消息
                    self.add_streaming_content(token);
                    // 自动滚动到底部（如果用户没手动滚动）
                    self.chat_scroll_offset = 0;
                }
            }

            if done {
                self.is_waiting_for_response = false;
            }

            // 如果还没结束，把 receiver 放回去
            if should_put_back {
                self.llm_receiver = Some(receiver);
            }
        
    }
}