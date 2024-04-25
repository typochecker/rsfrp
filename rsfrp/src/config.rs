/*!
 * Configuration module
 *
 * 用于加载配置文件到结构体，或者保存runtime配置到文件
 */
#![allow(unused)]

use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
struct CommonConfig {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    bindport: u16,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct ClientConfig {
    server_port: u16,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    common: CommonConfig,
    server: ServerConfig,
    client: ClientConfig,
}

impl Config {
    fn new() -> Config {
        todo!()
    }
    fn load_toml_from_file(path: &str) -> Config {
        todo!()
    }
    fn save_toml_to_file(&self, path: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config {
            common: CommonConfig {
                name: "rsfrp".to_string(),
                version: "0.1.0".to_string(),
            },
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                bindport: 8080,
                token: "server_token".to_string(),
            },
            client: ClientConfig {
                server_port: 8080,
                token: "server_token".to_string(),
            },
        };
        let serialized_config = toml::to_string(&config).unwrap();
        println!("Serialized config: \n{}", serialized_config);
    }
}
