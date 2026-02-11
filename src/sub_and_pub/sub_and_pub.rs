
use std::{io::Error, net::SocketAddr, path::PathBuf, sync::Arc, thread};
use tokio::{net::{TcpListener, TcpStream, tcp::WriteHalf}, sync::mpsc};

use crate::{
    handle_client::{SubscribeMessage, handle_client},
    store_message::store_message::store_message_in_file
};
    // TODO 
    // 1. backup 디렉토리 및 파일 생성 로직은 처음 어플리케이션 실행 시점에 한 번만 실행될 수 있도록 할 것
    // 2. 백업 파일에 데이터를 쓰는 로직 분리 필요함
    // 3. 메세지 Queue 생성 및 Queue에 데이터 삽입 로직 추가 필요함
    // 비동기적으로 파일 IO와 queeue 삽입 작업을 진행하고자 할 경우 tokio 크레이트 필요함
    // 메세지 큐로 들어온 메세지들을 몇개씩 묶어서 파일에 쓰는 작업을 진행 할 수 있으면 좋겠는데, 이럴려면 메세지들을 묶을 수 있는 단위의 벡터를 미리 생성해둘 필요가 있을 것 같음
    // 메세지를 묶을 단위의 벡터는 기동 시 프로퍼티 파일에 정의된 값에 따라 사이즈가 결졍될 수 있도록 할 것 

// 이쪽에서 메세지 IO 작업 진행하는 함수 CALL 하고 Message Queue도 만들어야 할듯
pub async fn sub_and_pub<T>(listner: TcpListener, file_path: PathBuf, mut message_queue: Vec<String>, mut message_store_vector: Vec<String>) -> Result<(), Error> {
    
    // clinet의 요청을 처리할 함수와 통신할 채널 생성
    let (tx, mut rx) = mpsc::unbounded_channel::<Option<SubscribeMessage>>();

    let tcp_listening_thread = tokio::spawn(async move {
        // TCP/IP 연결 수신 루프
        loop {
            if let Ok(accept_result) = listner.accept().await {
                let (mut socket, addr) = accept_result;
                let tx_clone = tx.clone();
                println!("Stream loop entered");
                handle_client::handle_connection(socket, addr, tx_clone).await;
                println!("serve_client called");
                
                
            } else {
                eprintln!("Failed to accept connection");
            }
        };
    });

    // 채널 수신 및 데이터 처리 로직 비동기 블럭 내에 생성
    let message_store_thread = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if message.is_none() {
                eprintln!("Failed to parse request body. Skipping this connection.");
                continue;
            } else {
                // TODO: 메세징 큐 변수로 데이터를 로드하는 로직이 필요함
                // 
                let msg = message.unwrap();
                message_queue.push(msg.get_data().clone());
                message_store_vector.push(msg.get_data().clone());

                println!("message_store_vector.len() == {}", message_store_vector.len());
                if message_store_vector.len() == 5 {
                    let v = message_store_vector.clone();
                    let fp = file_path.clone();
                    // 데이터를 구조체로 받아왔으면 파일에 저장도 해야지...
                    // 저장하고 그 다음에 store_vector 비워줘야함
                    // 그러기 위해서는 저장한 다음 저장에 성공했다는 사인을 전달 받을 필요가 있음
                    let store = tokio::spawn(async {
                        println!("첫번째 async 단위");
                        store_message_in_file(fp, v).await;
                    });
                        
                    let _ = store.await;
                    message_store_vector.clear();
                }
                println!("async 하고 그 다음에 출력이 되는지 봐야함");
            }
        }
    });


    let _result = tcp_listening_thread.await.unwrap_or_else(|f| {eprintln!("{}", f)});
    let _result2 = message_store_thread.await.unwrap_or_else(|f| {eprintln!("{}", f)});
    Ok(())
}