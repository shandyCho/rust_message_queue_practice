use std::{net::SocketAddr};
use tokio::{net::{tcp::{OwnedWriteHalf}}};

pub fn send_error_to_publisher(err_msg: String, mut socket: &OwnedWriteHalf, addr: SocketAddr) {
    let write_result = socket.try_write(err_msg.as_bytes());
    match write_result {
        Ok(_) => eprintln!("Send error to client is Success. System Will close TcpStream from {}", addr),
        Err(e) => eprintln!("Send error to client is failed with e: {}", e)
    };
}