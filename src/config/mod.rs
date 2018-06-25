
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::str::FromStr;
use toml::from_str;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{config, init_config};

#[derive(Deserialize)]
pub struct Log {
    pub file: String,
    pub level: String,
    pub pattern: Option<String>
}

#[derive(Deserialize)]
pub struct Mqtt {
    pub server: String,
    pub user: String,
    pub password: String,
    pub ca_path: Option<String>,
    pub topics: Vec<String>
}

#[derive(Deserialize)]
pub struct DB {
    pub url: String
}

#[derive(Deserialize)]
pub struct Config {
    pub log: Log,
    pub mqtt: Mqtt,
    pub db: DB
}

impl Config {
    pub fn new(location: &str) -> Config {
        let mut f = File::open(location).expect(format!("fail to open config file: {}", location).as_str());
        let mut content = String::new();

        f.read_to_string(&mut content).expect(format!("fail to read config file: {}", location).as_str());
        let config: Config = from_str(content.as_str()).expect("fail to parse config file");

        init_log_facility(&config);
        config
    }
}

fn init_log_facility(config: &Config) {
    let log = &config.log;
    let pattern = &log.pattern;
    let default_pattern = "{d} - {m}{n}".to_string();
    let pattern = pattern.as_ref().unwrap_or(&default_pattern);
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern.as_str())))
        .build(&log.file).expect("fail to init file appender");

    let mut config_builder = config::Config::builder()
        .appender(config::Appender::builder().build("file", Box::new(file_appender)));
    let mut root_builder = config::Root::builder().appender("file");

    // add console when debug build
    let profile = env::var("PROFILE").unwrap_or("debug".to_string());
    if profile == "debug" {
        let stdout = ConsoleAppender::builder().build();
        config_builder = config_builder.appender(config::Appender::builder().build("console", Box::new(stdout)));
        root_builder = root_builder.appender("console");
    }

    let level = LevelFilter::from_str(log.level.as_str())
        .expect(format!("invalid level: {}", log.level).as_str());
    let config = config_builder.build(root_builder.build(level)).expect("fail to init log config");

    let _handle = init_config(config).expect("fail to init log");
    // 暂时不支持动态修改
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_config() {
        let mut config_file: String = env!("CARGO_MANIFEST_DIR").to_string();
        config_file.push_str("/tests/test_config.toml");
        let config = Config::new(config_file.as_str());

        assert_eq!(config.db.url, "iot_center.db");

        assert_eq!(config.mqtt.server, "localhost:8883");
        assert_eq!(config.mqtt.ca_path, Some("/tmp/ca.pem".to_string()));

        assert_eq!(config.log.file, "output.log");
        assert_eq!(config.log.pattern, None);
    }
}