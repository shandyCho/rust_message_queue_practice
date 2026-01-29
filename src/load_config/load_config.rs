/// 최초 구동시 설정 yaml 파일을 읽어들이는 모듈

use std::{fs, path::{Path, PathBuf}};
use serde::Deserialize;

use crate::store_message::store_message::check_file_exists;

#[derive(Deserialize, Debug)]
struct RawConfig {
    host: String,
    port: u16,
    file_path: String
}      

pub struct InitialConfig {
    host: String,
    port: u16,
    file_path: PathBuf
}

impl RawConfig {
    pub fn get_host(&self) -> &String {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }    
}

impl InitialConfig {
    pub fn new(host: String, port: u16, file_path: PathBuf) -> InitialConfig {
        InitialConfig {
            host,
            port,
            file_path: file_path
        }
    }
    pub fn get_host(&self) -> &String {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

pub fn load_config() -> InitialConfig {
             
    // let config_yaml = fs::read_to_string("/etc/test_mq/test_config.yaml")
    //                             .expect("Fail to find config file /etc/test_mq/test_config.yaml");
    let config_yaml = fs::read_to_string("./test_config.yaml")
                                .expect("Fail to find config file /etc/test_mq/test_config.yaml");
    let deserialized_yaml: RawConfig = serde_saphyr::from_str(config_yaml.as_str())
                                .expect("yaml structure isn't correct");

    println!("Config loaded: {:?}", deserialized_yaml);
    let file_path = check_file_exists(PathBuf::from(deserialized_yaml.get_file_path().to_string()));
    InitialConfig::new(
        deserialized_yaml.get_host().to_string(),
        deserialized_yaml.get_port(),
        file_path
    )
    
}
