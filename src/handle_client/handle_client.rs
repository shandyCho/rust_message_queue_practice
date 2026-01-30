/// 메세지 큐는 따로 HTTP Body에 담긴 데이터를 처리할 필요가 없기 때문에 타겟으로 지정된 어드레스로 보내는 것만 신경 써볼것
use core::str;
// use std::net::TcpStream;
use std::io::{BufReader, Read};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc};
use tokio_util::codec::{FramedRead, LinesCodec};
use tokio_stream::StreamExt;

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

pub async fn handle_connection(mut socket: TcpStream, tx: mpsc::UnboundedSender<Option<SubscribeMessage>>) {
    // 줄바꿈 문자 (\n) 기준으로 데이터를 읽어들이는 Reader 생성
    // LinesCodec 의 기본 설정이 줄바꿈 문자로 데이터를 구분하는 것임
    let mut framed_reader = FramedRead::new(socket, LinesCodec::new());
    while let Some(result) = framed_reader.next().await {
        match result {
            Ok(line) => {
                match serde_json::from_str::<SubscribeMessage>(&line) {
                    Ok(parsed) => {
                        println!("sender_address: {:?}, data: {:?}", &parsed.get_sender_address(), &parsed.get_data());
                        tx.send(Some(parsed));
                    }
                    Err(e) => {
                        println!("Failed to parse JSON: {}", e);
                        println!("Converted Data: {}", &line);
                        tx.send(None);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                tx.send(None);
            }
        }
    };
    // result_vector




    // let mut body = Vec::new();
    // reader.read_to_end(&mut body).ok();
    // let converted_data = match str::from_utf8(&body) {
    //     Ok(v) => v.to_owned(),
    //     Err(e) => {
    //         println!("Failed to convert body to UTF-8 string: {}", e);
    //         return None;
    //     }
    // };
    // println!("{}", converted_data);
    // let data = match serde_json::from_str::<SubscribeMessage>(&converted_data) {
    //     Ok(parsed) => {
    //         println!("sender_address: {:?}, data: {:?}", &parsed.get_sender_address(), &parsed.get_data());
    //         Some(parsed)
    //     }
    //     Err(e) => {
    //         println!("Failed to parse JSON: {}", e);
    //         println!("Converted Data: {}", converted_data);
    //         None
    //     }
    // };
    // data   
}