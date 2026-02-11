use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Message {
    is_published: bool,
    subject: String,
    data: Value,
}

impl Message {
    pub fn is_published(&self) -> bool {
        let is_published = &self.is_published;
        (*is_published).clone()
    }
    pub fn get_subject(&self) -> String {
        let subject = &self.subject;
        (*subject).clone()
    }

    pub fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
    pub fn new(is_published: bool, subject: String, data: Value) -> Self {
        Self { is_published, subject, data }
    }
}


pub struct PublishedMessage {
    subject: String,
    data: String
}

impl PublishedMessage {
        pub fn get_subject(&self) -> &String {
        &self.subject
    }

    pub fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
    pub fn new(subject: String, data: String) -> Self {
        Self { subject, data }
    }
}