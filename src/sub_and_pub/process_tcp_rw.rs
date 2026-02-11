use std::net::SocketAddr;

use tokio::{net::TcpStream, sync::mpsc};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::{handle_publisher::handle_publisher::send_error_to_publisher, sub_and_pub::message_structs::{Message, PublishedMessage}};

pub async fn proccess_tcp_read_and_write(mut socket: TcpStream, addr: SocketAddr, tx: mpsc::UnboundedSender<Option<PublishedMessage>>) {
    // Read Write Stream을 분할하여 사용한다
    let (r, w) = socket.into_split();
    
    // TCP Stream을 한 줄씩 파싱해서 처리한다 
    let mut frame_reader = FramedRead::new(r, LinesCodec::new());
    'frame_reading: while let Some(result) = frame_reader.next().await {
        // 읽어온 Stream이 정상적으로 String으로 변환되었으면 그 값을 line에 담고, 
        // 아니라면 그만 읽고 루프를 빠져나온다
        let line: String;
        match result {
            Ok(success) => { line = success }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                tx.send(None)
                .unwrap_or_else(|e| {
                    eprintln!("Fail to send message to another thread with LinesCodecError with e: {}", e);
                });
                break 'frame_reading;
            }
        }

        let deserialized_result = serde_json::from_str::<Message>(&line);
        // String으로 변환된 line을 Message 객체로 변환한다
        match deserialized_result {
            Ok(message) => {
                if message.is_published() {
                    let published_message = PublishedMessage::new(
                        message.get_subject(), 
                        message.get_data()
                    );
                    tx.send(Some(published_message))
                        .unwrap_or_else(|e| {
                            eprintln!("Parsing Messege is Success But fail to send message to another thread with e: {}", e);
                        });
                } else {
                    // 여기서 serve_scriber를 호출해야할듯?
                }
                
            }
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                eprintln!("Converted Data: {}", &line);
                send_error_to_publisher("Failed to parse request body.".to_string(),  &w, addr);
                
                let _tx_result = tx.send( None).unwrap_or_else(|e| {
                    eprintln!("Fail to send message to another thread with e: {}", e);
                });
                break 'frame_reading;
            }
        }

    }
}