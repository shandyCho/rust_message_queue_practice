/// 메세지 큐는 따로 HTTP Body에 담긴 데이터를 처리할 필요가 없기 때문에 타겟으로 지정된 어드레스로 보내는 것만 신경 써볼것
use core::str;
use std::{net::SocketAddr};

use serde::{Deserialize, Serialize};
use serde_json::{Value, Error};
use tokio::{net::{TcpStream, tcp::{OwnedWriteHalf}}, sync::mpsc};
use tokio_util::codec::{FramedRead, LinesCodec};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Message {
    is_published: bool,
    subject: String,
    data: Value,
}

impl Message {
    pub fn is_published(&self) -> &bool {
        &self.is_published
    }
    pub fn get_subject(&self) -> &String {
        &self.subject
    }

    pub fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
}


pub async fn read_publisher_message(mut socket: TcpStream, addr: SocketAddr, tx: mpsc::UnboundedSender<Option<Message>>) {
    // 줄바꿈 문자 (\n) 기준으로 데이터를 읽어들이는 Reader 생성
    // LinesCodec 의 기본 설정이 줄바꿈 문자로 데이터를 구분하는 것임
    let (r, w) = socket.into_split();

    let mut framed_reader = FramedRead::new(r, LinesCodec::new());
    'frame_reading: while let Some(result) = framed_reader.next().await {
        match result {
            Ok(line) => {
                if line == "publish" {

                }
                let parsed: Message = serde_json::from_str::<Message>(&line)
                    .unwrap_or_else(|e: Error| {
                        eprintln!("Failed to parse JSON: {}", e);
                        eprintln!("Converted Data: {}", &line);
                        send_error_to_publisher("Failed to parse request body.".to_string(),  &w, addr);
                        
                        let _tx_result = tx.send( None).unwrap_or_else(|e| {
                            eprintln!("Fail to send message to another thread with e: {}", e);
                        });
                        
                        Message {
                            subject: "".to_string(), 
                            data: serde_json::Value::String(String::new()) 
                        }
                    }); 
                
                if !parsed.get_subject().is_empty() {
                    println!("sender_address: {:?}, data: {:?}", &parsed.get_subject(), &parsed.get_data());
                    tx.send(Some(parsed))
                    .unwrap_or_else(|e| {
                        eprintln!("Parsing Messege is Success But fail to send message to another thread with e: {}", e);
                    });
             
                } else {
                    // 메세지 파싱에서 에러가 발생할 경우, 반복문을 나와서 나머지 데이터들을 버려줄 필요가 있다
                    println!("parsing error");
                    break 'frame_reading;
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                tx.send((None)).unwrap_or_else(|e| {
                    eprintln!("Fail to send message to another thread with LinesCodecError with e: {}", e);
                });
                break 'frame_reading;
            }
        };
    };
}

fn send_error_to_publisher(err_msg: String, mut socket: &OwnedWriteHalf, addr: SocketAddr) {
    let write_result = socket.try_write(err_msg.as_bytes());
    match write_result {
        Ok(_) => eprintln!("Send error to client is Success. System Will close TcpStream from {}", addr),
        Err(e) => eprintln!("Send error to client is failed with e: {}", e)
    };
}