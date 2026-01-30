use std::{
    fs::{
        self, 
        File
    }, 
    io::{
        BufWriter, 
        Read, 
        Write
    }, 
    path::PathBuf
};
use tokio::{
    fs::{
        self as tokio_fs,
        File as AsyncFile
    },
    io::{
        BufWriter as AsyncBufWriter,
        AsyncWriteExt
    }
};


pub fn check_file_exists(file_path: PathBuf) -> PathBuf {
    println!("path exists: {}", file_path.exists());
    println!("is file: {}", file_path.is_file());

    // 파일 경로가 존재하고 해당 경로가 파일일 경우라면 Path 그대로 반환함
    // 그것이 아니라면 생성작업 진행. 생성 실패 시 프로그램을 종료한다.
    if file_path.exists() && file_path.is_file() {
        println!("Backup file exists.");
        file_path
    } else {
        println!("Backup directory does not exist or is not a directory.");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        let file = File::create(&file_path);
        match file {
            Ok(_) => {
                println!("Backup file created successfully.");
                file_path
            },
            Err(e) => {
                println!("Failed to create backup file: {}", e);
                panic!("Cannot proceed without backup file.");
            }
        }
    }
}


pub async fn store_message_in_file(file_path: PathBuf, message_vector: Vec<String>) {
    // message_vector 의 경우 clone 한 것을 사용하도록 해야?
    // 아니지 어짜피 파일 IO 용으로 새로 vector를 만드는거니 괜찮을듯
    println!("message saving....");
    let backup_file = AsyncFile::options()
        .append(true)
        .open(file_path).await;
    
    match backup_file {
        Ok(file) => {
            let mut message_form_vector = String::new();
            let mut buf = AsyncBufWriter::new(file);

            for message in message_vector {
                message_form_vector = message_form_vector + (message + "\n").as_str();
            }
            let result = match buf.write_all(message_form_vector.as_bytes())
                .await {
                    Ok(_) => {
                        buf.flush().await;
                        // Ok(())
                    },
                    Err(e) => {
                        eprintln!("Failed to write messages to file: {}", e);
                        return;
                    }
                };
            
        },
        Err(e) => {
            eprintln!("Failed to open file for appending: {}", e);
            return;
        }
    };



}
