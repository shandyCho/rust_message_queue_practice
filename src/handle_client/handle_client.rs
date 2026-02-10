/// 메세지 큐는 따로 HTTP Body에 담긴 데이터를 처리할 필요가 없기 때문에 타겟으로 지정된 어드레스로 보내는 것만 신경 써볼것
use core::str;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Value, Error};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpSocket, TcpStream}, sync::mpsc};
use tokio_util::codec::{FramedRead, LinesCodec};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SubscribeMessage {
    classifier: String,
    data: Value,
}

impl SubscribeMessage {
    pub fn get_classifier(&self) -> &String {
        &self.classifier
    }

    pub fn get_data(&self) -> String {
        let stringfied_data = &self.data.to_string();
        (*stringfied_data).clone()
    }
}

pub async fn handle_connection(mut socket: TcpStream, tx: mpsc::UnboundedSender<Option<(TcpSocket, SubscribeMessage)>>) {
    // 줄바꿈 문자 (\n) 기준으로 데이터를 읽어들이는 Reader 생성
    // LinesCodec 의 기본 설정이 줄바꿈 문자로 데이터를 구분하는 것임
    let mut framed_reader = FramedRead::new(socket, LinesCodec::new());
    while let Some(result) = framed_reader.next().await {
        match result {
            Ok(line) => {
                let parsed: SubscribeMessage = serde_json::from_str::<SubscribeMessage>(&line)
                    .unwrap_or_else(|e: Error| {
                        eprintln!("Failed to parse JSON: {}", e);
                        eprintln!("Converted Data: {}", &line);
                        tx.send((socket, None));
                        SubscribeMessage {
                            classifier: "".to_string(), 
                            data: serde_json::Value::String(String::new()) 
                        }
                    }); 
                
                if !parsed.get_classifier().is_empty() {
                    println!("sender_address: {:?}, data: {:?}", &parsed.get_classifier(), &parsed.get_data());
                    tx.send(Some(parsed));
             
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                tx.send(None);
            }
        };
    };
}

pub async fn send_error_to_client(err_msg: String, listner: Arc<TcpListener>) {

    if let Ok(accept_result) = listner.accept().await {
        let (mut socket, _addr) = accept_result;
        socket
            .write_all("Server can not parse client message".as_bytes())
            .await
            .inspect_err(|err| println!("Can not send error message to client: {}", err));
    } else {
            eprintln!("Failed to accept connection for sending error message: {}", err_msg);
    };
}