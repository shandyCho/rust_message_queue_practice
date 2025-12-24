/// 메세지 큐는 따로 HTTP Body에 담긴 데이터를 처리할 필요가 없기 때문에 타겟으로 지정된 어드레스로 보내는 것만 신경 써볼것
use core::str;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, Read};
use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequestBody<T> {
    sender_address: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(bound(deserialize = "T: FromStr, T::Err: Display"))]
    data: T,
}

fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    T::from_str(s).map_err(serde::de::Error::custom)
}



impl<T> HttpRequestBody<T> {
    pub fn get_sender_address(&self) -> &String {
        &self.sender_address
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 100];
    while stream.read(&mut buffer).is_err() {
        println!("Reading from client...");
    }
    println!("Client sent: {:?}", String::from_utf8_lossy(&buffer));
}


pub fn handle_connection<T: FromStr> (mut stream: TcpStream) -> HttpRequestBody<String> {
    let mut reader = BufReader::new(&mut stream);
    let mut request_per_line = String::new();
    let mut headers = Vec::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;  // Connection Closed
        }
        if line.trim().is_empty() {
            // headers와 body를 나누는 빈 라인을 찾았을 때
            break;
        }
        if line.starts_with("Content-Length:") {
            if let Some(len_str) = line.split(":").nth(1) {
                content_length = len_str.trim().parse::<usize>().unwrap_or(0);
            }
        }
        headers.push(line);
    } 

    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).unwrap();
    let request_body = str::from_utf8(&body).unwrap();
    // println!("Request Body: {}", request_body);

    // let request_body = request_body.to_string();

    let request_body_structure: HttpRequestBody<String> = serde_json::from_str(request_body)
        .expect("Failed to parse JSON from request body");
    println!("Request Body: {:?}", request_body_structure);
    request_body_structure
}