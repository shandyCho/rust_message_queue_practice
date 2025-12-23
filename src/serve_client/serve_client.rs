use std::net::TcpStream;
use std::io::Write;
use crate::handle_client::HttpRequestBody;

pub fn serve_client(request_body: HttpRequestBody) {
    let mut stream = TcpStream::connect(request_body.get_sender_address())
        .expect("Could not connect to target address");
    stream.write_all(request_body.get_data().as_bytes())
        .expect("Failed to send data to target address");
}