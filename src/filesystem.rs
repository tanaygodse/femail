use crate::gmail::{GmailClient, Label, Message};
use anyhow::Result;

pub struct GmailFilesystem {
    gmail_client: GmailClient,
}

impl GmailFilesystem {
    pub fn new(gmail_client: GmailClient) -> Self {
        Self { gmail_client }
    }
    
    pub fn list_labels(&self) -> Result<Vec<Label>> {
        self.gmail_client.list_labels()
    }
    
    pub fn list_messages(&self, label_id: &str) -> Result<Vec<Message>> {
        self.gmail_client.list_messages(label_id)
    }
    
    pub fn read_message(&self, message_id: &str) -> Result<String> {
        self.gmail_client.get_message_content(message_id)
    }
}