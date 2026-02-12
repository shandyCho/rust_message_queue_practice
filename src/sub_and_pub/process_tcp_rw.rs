use std::net::SocketAddr;

use tokio::{net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, sync::mpsc};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::{sub_and_pub::message_structs::{ChannelMessage, FrameReadedMessage, MessageDefaultGetter, PublishedMessage}};

pub async fn proccess_tcp_read(
    mut read_stream: OwnedReadHalf, 
    addr: SocketAddr, 
    message_tx: mpsc::UnboundedSender<Option<PublishedMessage>>, 
    subscriber_tx: mpsc::UnboundedSender<ChannelMessage>) {

    // TCP Stream을 한 줄씩 파싱해서 처리한다 
    let mut frame_reader = 
        FramedRead::new(read_stream, LinesCodec::new());

    'frame_reading: while let Some(result) = frame_reader.next().await {
        // 읽어온 Stream이 정상적으로 String으로 변환되었으면 그 값을 line에 담고, 
        // 아니라면 그만 읽고 루프를 빠져나온다
        let line: String;
        match result {
            Ok(success) => { line = success }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                message_tx.send(None)
                .unwrap_or_else(|e| {
                    eprintln!("Fail to send message to another thread with LinesCodecError with e: {}", e);
                });
                break 'frame_reading;
            }
        }

        let deserialized_result = serde_json::from_str::<FrameReadedMessage>(&line);
        // String으로 변환된 line을 Message 객체로 변환한다
        match deserialized_result {
            Ok(message) => {
                // Message가 publisher에게서 온 것이라면 메세지 저장을 위해 메세지를 message_store_thread 으로 보냄
                if message.is_published() {
                    let published_message = PublishedMessage::new(
                        message.get_subject(), 
                        message.get_data()
                    );
                    message_tx.send(Some(published_message))
                        .unwrap_or_else(|e| {
                            eprintln!("Parsing Messege is Success But fail to send message to another thread with e: {}", e);
                        });
                // Message가 subscriber에게서 온 것이라면 메세지 전송이 필요함....        
                } else {
                    // 여기서 serve_scriber를 호출해야할듯?
                    let channel_message = ChannelMessage::new(
                        false, 
                        "".to_string(), 
                        message.get_subject()
                    );
                    subscriber_tx.send(channel_message)
                        .unwrap_or_else(|e| {
                            eprintln!("Parsing Messege is Success But fail to send message to another thread with e: {}", e);
                        });

                }
                
            }
            // Message 객체로 변환이 되지 않을 경우 스트림을 그만 읽고 루프에서 빠져나온다
            Err(e) => {
                let channel_message = ChannelMessage::new(
                    true, 
                    format!("Failed to parse JSON: {}, Converted Data: {}", e, &line),
                    "".to_string());
                let _tx_result = subscriber_tx.send(channel_message).unwrap_or_else(|e| {
                    eprintln!("Fail to send message to another thread with e: {}", e);
                });
                break 'frame_reading;
            }
        }

    }
}


pub async fn process_tcp_write(subject: String, mut write_stream: &OwnedWriteHalf, addr: SocketAddr) {
    todo!("이 곳에서 메세징 큐를 읽거나 메세징 저장한 파일을 읽어서 보내도록 하기")
}