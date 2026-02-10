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
    let path = config.get_file_path().to_path_buf();
    let mut message_queue: Vec<String> = Vec::new();
    let mut message_store_vector: Vec<String> = Vec::new();
    sub_and_pub::<String>(listner, path, message_queue, message_store_vector).await?;
    Ok(())
}