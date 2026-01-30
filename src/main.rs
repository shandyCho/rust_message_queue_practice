/// 메세징 큐 시스템을 만들기 위해서는 TCP 연결을 받아서 데이터를 처리하는 쪽과 처리한 데이터를 클라이언트에게 전달하는 쪽으로 나뉘어야 한다.
/// 이를 위해서는 멀티 스레딩 혹은 비동기 프로그래밍을 써야할 것 같다.
/// 아닌가? 단순히 TCP 연결을 받아서 데이터를 처리하고 다시 보내주는 것이라면 싱글 스레드로도 가능할 것 같다.
/// 단순히 중계 서버 역할만 해준다면 싱글 스레드로 처리가 되겠으나, 메세지 유실 방지를 위해 인덱싱 혹은 메세지 저장을 하게 된다면 아무래도 멀티 스레딩을 할 수 밖에 없을 것 같다.

pub mod load_config;
pub mod handle_client;
pub mod serve_client;
pub mod sub_and_pub;
pub mod store_message;
use std::{error::Error};

use load_config::InitialConfig;
use tokio::net::TcpListener;

use crate::sub_and_pub::sub_and_pub::sub_and_pub;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, Message Queue!");

    let config: InitialConfig = load_config::load_config::load_config();

    let addr = format!("{}:{}", config.get_host(), config.get_port());
    let listner = TcpListener::bind(&addr).await?;
        // .expect(format!("Could not bind to address {}", &addr).as_str());
    let path = config.get_file_path().to_path_buf();
    let mut message_queue: Vec<String> = Vec::new();
    let mut message_store_vector: Vec<String> = Vec::new();
    sub_and_pub::<String>(listner, path, message_queue, message_store_vector).await?;
    Ok(())
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // 1. 서버 B의 8080 포트에서 수신 대기
//     let addr = "0.0.0.0:8080";
//     let listener = TcpListener::bind(addr).await?;
//     println!("서버 B가 {}에서 대기 중입니다...", addr);

//     loop {
//         // 2. 새로운 연결(A 또는 C)이 들어올 때까지 대기
//         let (mut socket, addr) = listener.accept().await?;
//         println!("새로운 연결 감지: {}", addr);

//         // 3. tokio::spawn을 사용하여 각 연결을 별도의 비동기 태스크로 분리
//         // 이 덕분에 A의 데이터를 읽는 동안 C의 데이터도 동시에 읽을 수 있습니다.
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             loop {
//                 match socket.read(&mut buf).await {
//                     // 데이터 읽기 성공
//                     Ok(n) if n > 0 => {
//                         let msg = String::from_utf8_lossy(&buf[..n]);
//                         println!("[{}]로부터 받은 데이터: {}", addr, msg);
                        
//                         // 응답 보내기 (선택 사항)
//                         if let Err(e) = socket.write_all(b"ACK").await {
//                             eprintln!("응답 전송 실패: {}", e);
//                             break;
//                         }
//                     }
//                     Ok(_) => {
//                         println!("연결 종료: {}", addr);
//                         break;
//                     }
//                     Err(e) => {
//                         eprintln!("데이터 읽기 오류 ({}): {}", addr, e);
//                         break;
//                     }
//                 }
//             }
//         });
//     }
// }
