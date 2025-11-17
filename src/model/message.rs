use crate::model::role::Role;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self {role, content, timestamp: Local::now()}
    }
}