/// 메세지 큐는 따로 HTTP Body에 담긴 데이터를 처리할 필요가 없기 때문에 타겟으로 지정된 어드레스로 보내는 것만 신경 써볼것
use core::str;
use std::fs::{self, File};
use std::net::TcpStream;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

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
        stringfied_data.clone()
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Option<SubscribeMessage> {
    let mut reader = BufReader::new(&mut stream);
    // let mut headers = Vec::new();
    // let mut content_length = 0;

    
    let mut body = Vec::new();
    reader.read_to_end(&mut body).ok();
    let converted_data = match str::from_utf8(&body) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to convert body to UTF-8 string: {}", e);
            return None;
        }
    };
    // reader.read_exact(&mut body).unwrap();
    // let request_body = str::from_utf8(&body).unwrap();
    println!("{}", converted_data);
    let path = Path::new("./msqTest/backup");
    match serde_json::from_str::<SubscribeMessage>(converted_data) {
        Ok(parsed) => {
            println!("sender_address: {:?}, data: {:?}", &parsed.get_sender_address(), &parsed.get_data());
            println!("path exists: {}", path.exists());
            println!("is file: {}", path.is_file());
            if path.exists() && path.is_file() {
                let file = File::options()
                    .append(true)
                    .open(path);
                let mut buf = BufWriter::new(file.unwrap());
                buf.write_all(converted_data.as_bytes()).unwrap();
                buf.write_all(b"\n").unwrap();
                buf.flush().unwrap();
            } else {
                println!("Backup directory does not exist or is not a directory.");
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                let file = File::create(path);
                match file {
                    Ok(_) => {
                        println!("Backup file created successfully.");
                        let mut buf = BufWriter::new(file.unwrap());
                        buf.write_all(converted_data.as_bytes()).unwrap();
                        buf.write_all(b"\n").unwrap();
                        buf.flush().unwrap();
                    },
                    Err(e) => {
                        println!("Failed to create backup file: {}", e);
                        return Some(parsed);
                    }
                }
            }
            Some(parsed)
        }
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            println!("Converted Data: {}", converted_data);
            None
        }
    }
}



// HTTP를 가정하고 구현하였으나 TCP 프로토콜을 사용하는 방식으로 변경해야함
// pub fn handle_connection(mut stream: TcpStream) -> Option<HttpRequestBody> {
//     let mut reader = BufReader::new(&mut stream);
//     let mut headers = Vec::new();
//     let mut content_length = 0;

//     loop {
//         let mut line = String::new();
//         if reader.read_line(&mut line).unwrap() == 0 {
//             break;  // Connection Closed
//         }
//         if line.trim().is_empty() {
//             // headers와 body를 나누는 빈 라인을 찾았을 때
//             break;
//         } else {
//             println!("Header Line: {}", &line.trim());
//         }
//         if line.starts_with("Content-Length:") {
//             if let Some(len_str) = line.split(":").nth(1) {
//                 content_length = len_str.trim().parse::<usize>().unwrap_or(0);
//             }
//         }
//         headers.push(line);
//     } 

//     let mut body = vec![0; content_length];
//     reader.read_exact(&mut body).unwrap();
//     let request_body = str::from_utf8(&body).unwrap();
//     println!("{}", request_body);

//     match serde_json::from_str::<HttpRequestBody>(request_body) {
//         Ok(parsed) => {
//             println!("sender_address: {:?}, data: {:?}", &parsed.get_sender_address(), &parsed.get_data());
//             Some(parsed)
//         }
//         Err(e) => {
//             println!("Failed to parse JSON: {}", e);
//             None
//         }
//     }
// }

