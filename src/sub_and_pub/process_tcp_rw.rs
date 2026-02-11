use std::net::SocketAddr;

use tokio::{net::TcpStream, sync::mpsc};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::handle_publisher::Message;

pub async fn proccess_tcp_read_and_write(mut socket: TcpStream, addr: SocketAddr, tx: mpsc::UnboundedSender<Option<Message>>) {
    let (r, w) = socket.into_split();
    let mut frame_reader = FramedRead::new(r, LinesCodec::new());
    'frame_reading: while let Some(result) = frame_reader.next().await {
        match result {
            Ok(line) => {
                if line == "publish" {

                }
                if line == "subscribe" {

                }
            }
            Err(e) => {

            }
        }
    }
}