use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait MessageDefaultGetter {
    fn get_subject(&self) -> String;
    fn get_data(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct FrameReadedMessage
 {
    is_published: bool,
    subject: String,
    data: Value,
}

impl FrameReadedMessage {
    pub fn is_published(&self) -> bool {
        let is_published = &self.is_published;
        (*is_published).clone()
    }
    pub fn new(is_published: bool, subject: String, data: Value) -> Self {
        Self { is_published, subject, data }
    }
}

impl MessageDefaultGetter for FrameReadedMessage {
    fn get_subject(&self) -> String {
        let subject = &self.subject;
        (*subject).clone()
    }

    fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
}

#[derive(Debug)]
pub struct PublishedMessage {
    subject: String,
    data: String
}

impl PublishedMessage {
    pub fn new(subject: String, data: String) -> Self {
        Self { subject, data }
    }
}

impl MessageDefaultGetter for PublishedMessage {
    fn get_subject(&self) -> String {
        let subject = &self.subject;
        (*subject).clone()
    }

    fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
}


// ChannelMessage는 에러 메세지를 채널로 전달하거나, 데이터 전송 요청 정보를 채널로 전달하는데 써야함 
#[derive(Debug, Clone)]
pub struct ChannelMessage {
    is_error_message: bool,
    error_message: String,
    subject: String
}

impl ChannelMessage {
    pub fn is_error_message (&self) -> bool {
        let is_error_message = &self.is_error_message;
        (*is_error_message).clone()
    }
    pub fn get_error_message(&self) -> String {
        let error_message = &self.error_message;
        (*error_message).clone()
    }
    pub fn get_subject(&self) -> String {
        let subject = &self.subject;
        (*subject).clone()
    }
    pub fn new(is_error_message: bool, error_message: String, subject: String) -> Self {
        Self {  is_error_message, error_message, subject }
    }
}