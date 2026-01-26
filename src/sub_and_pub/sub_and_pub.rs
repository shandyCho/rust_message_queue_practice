
use std::net::TcpListener;
use crate::{
    handle_client::handle_client, 
    serve_client::serve_client
};

// pub fn pub_and_sub<T>(listner: TcpListener) {
//         for stream in listner.incoming() {
//         let request_body: HttpRequestBody<String> = handle_client::handle_connection::<String>(stream.unwrap());
//         serve_client::serve_client(request_body);
//     }
// }

pub fn sub_and_pub<T>(listner: TcpListener) {
        for stream in listner.incoming() {
            println!("Stream loop entered");
            match stream {
            Ok(mut s) => {
                println!("handle_client called");
                // handle_connection이 Option이나 Result를 반환하도록 수정하는 것이 좋습니다.
                // 여기서는 기존 로직을 유지하되 최소한의 안전장치를 고려합니다.
                let request_body = handle_client::handle_connection(s);
                println!("serve_client called");

                if request_body.is_none() {
                    eprintln!("Failed to parse request body. Skipping this connection.");
                    continue; // 다음 연결로 넘어갑니다.
                }
                // 데이터를 처리하는 로직
                serve_client::serve_client(request_body.unwrap());
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                // 연결 하나가 실패해도 루프를 계속 돌아야 서버가 유지됩니다.
                continue;
            }
        }


        // let request_body: HttpRequestBody = handle_client::handle_connection(stream.unwrap());
        // serve_client::serve_client(request_body);
    }
}