/// 메세지 큐는 따로 HTTP Body에 담긴 데이터를 처리할 필요가 없기 때문에 타겟으로 지정된 어드레스로 보내는 것만 신경 써볼것
use core::str;
use std::net::TcpStream;
use std::io::{BufReader, Read};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SubscribeMessage {
    sender_address: String,
    data: Value,
}

impl SubscribeMessage {
    pub fn get_sender_address(&self) -> &String {
        &self.sender_address
    }

    pub fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Option<SubscribeMessage> {
    let mut reader = BufReader::new(&mut stream);
    let mut body = Vec::new();
    reader.read_to_end(&mut body).ok();
    let converted_data = match str::from_utf8(&body) {
        Ok(v) => v.to_owned(),
        Err(e) => {
            println!("Failed to convert body to UTF-8 string: {}", e);
            return None;
        }
    };
    println!("{}", converted_data);
    let data = match serde_json::from_str::<SubscribeMessage>(&converted_data) {
        Ok(parsed) => {
            println!("sender_address: {:?}, data: {:?}", &parsed.get_sender_address(), &parsed.get_data());
            Some(parsed)
        }
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            println!("Converted Data: {}", converted_data);
            None
        }
    };
    data   
}