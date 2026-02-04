use std::net::TcpStream;
use std::io::Write;
use crate::handle_client::SubscribeMessage;

// pub fn serve_client(request_body: HttpRequestBody<String>) {
//     let mut stream = TcpStream::connect(request_body.get_sender_address())
//         .expect("Could not connect to target address");
//     stream.write_all(request_body.get_data().as_bytes())
//         .expect("Failed to send data to target address");
// }

pub fn serve_client(request_body: SubscribeMessage) {
    // let mut stream = TcpStream::connect(request_body.get_sender_address())
    //     .expect("Could not connect to target address");
    // stream.write_all(request_body.get_data().as_bytes())
    //     .expect("Failed to send data to target address");

    match TcpStream::connect::<&String>(request_body.get_classifier()) {
        Ok(mut stream) => {
            println!("Connected to target address: {}", request_body.get_classifier());
            stream.write_all(request_body.get_data().as_bytes())
                .expect("Failed to send data to target address");
        }
        Err(e) => {
            eprintln!("Could not connect to target address: {}", e);
        }
    }
}