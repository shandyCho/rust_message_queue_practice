/// 최초 구동시 설정 yaml 파일을 읽어들이는 모듈

use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InitialConfig {
    host: String,
    port: u16
}      

impl InitialConfig {
    pub fn get_host(&self) -> &String {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

pub fn load_config() -> InitialConfig {
             
    // let config_yaml = fs::read_to_string("/etc/test_mq/test_config.yaml")
    //                             .expect("Fail to find config file /etc/test_mq/test_config.yaml");
    let config_yaml = fs::read_to_string("./test_config.yaml")
                                .expect("Fail to find config file /etc/test_mq/test_config.yaml");
    let deserialized_yaml: InitialConfig = serde_saphyr::from_str(config_yaml.as_str())
                                .expect("yaml structure isn't correct");

    println!("Config loaded: {:?}", deserialized_yaml);
    deserialized_yaml
}
