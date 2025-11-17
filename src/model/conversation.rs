use uuid::Uuid;
use crate::model::message::Message;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}