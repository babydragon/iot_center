
extern crate rusqlite;
extern crate rumqtt;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chan_signal;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate toml;

mod config;
mod storage;
mod mqtt;

use std::env;
use std::rc::Rc;
use config::Config;
use storage::{IotMessage, Sink};
use mqtt::Mqtt;
use chan_signal::Signal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = &args[1];
    // init config
    println!("read config file {}", config_file);
    let config = Rc::new(Config::new(config_file.as_str()));

    // init db connection
    debug!("start to init db");
    let sink = Sink::new(config.clone());
    info!("finish to init db");

    // init mqtt client
    debug!("start to init mqtt");
    let _mqtt_client = Mqtt::new(config.clone(), move |message: &str| {
        let iot_message = match IotMessage::from_string(message) {
            Ok(m) => m,
            Err(e) => {
                error!("fail to parse mqtt message: {}", e);
                return;
            }
        };

        sink.save(&iot_message);
    });
    info!("finish to init mqtt");

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    signal.recv().unwrap();
}
