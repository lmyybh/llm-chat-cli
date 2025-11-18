#![allow(dead_code)]
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{Local};
use core::fmt::{Display, Formatter, Result};  

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl Display for Role {  
    fn fmt(&self, f: &mut Formatter) -> Result {  
        match self {  
            Role::User => write!(f, "User"),  
			Role::Assistant => write!(f, "Assistant"),
		}  
    }  
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing)]
    pub timestamp: String,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role, 
            content, 
            timestamp: Local::now().format("%H:%M:%S").to_string()
        }
    }
}


#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub model: String,
    pub api_url: String,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            model: "default".to_string(),
            api_url: "http://localhost:8000/v1/chat/completions".to_string(),
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}


#[derive(Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(flatten)]
    pub sampling_params: SamplingParams,
}

#[derive(Serialize, Debug)]
pub struct SamplingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
}