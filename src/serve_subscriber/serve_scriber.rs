use std::net::TcpStream;
use std::io::Write;

use crate::sub_and_pub::message_structs::Message;


pub fn serve_message_to_scriber(request_body: Message) {

    match TcpStream::connect::<String>(request_body.get_subject()) {
        Ok(mut stream) => {
            println!("Connected to target address: {}", request_body.get_subject());
            stream.write_all(request_body.get_data().as_bytes())
                .expect("Failed to send data to target address");
        }
        Err(e) => {
            eprintln!("Could not connect to target address: {}", e);
        }
    }
}