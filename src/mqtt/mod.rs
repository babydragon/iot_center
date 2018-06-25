
use rumqtt::{MqttOptions, MqttClient, QoS, MqttCallback, Message};
use super::config;
use std::path::Path;
use std::sync::Arc;
use std::rc::Rc;

pub struct Mqtt {
    client: MqttClient
}

impl Mqtt {

    pub fn new<F>(config: Rc<config::Config>, callback: F) -> Mqtt
        where F: Fn(&str) + Sync + Send + 'static
    {
        let mqtt_config = &config.mqtt;
        let mut client_options = MqttOptions::new()
            .set_keep_alive(5)
            .set_reconnect(3)
            .set_broker(mqtt_config.server.as_str())
            .set_user_name(mqtt_config.user.as_str())
            .set_password(mqtt_config.password.as_str())
            .set_should_verify_ca(false);

        if mqtt_config.ca_path.is_some() {
            client_options = client_options.set_ca(Path::new(&mqtt_config.ca_path.as_ref().unwrap()));
        }

        let mut mqtt_callback = MqttCallback::new();
        mqtt_callback = mqtt_callback.on_message(move |message: Message| {
            let message_content = match Arc::try_unwrap(message.payload) {
                Ok(content) => content,
                Err(_) => {
                    error!("fail to fetch byte array from mqtt message");
                    return;
                }
            };
            let payload_str = String::from_utf8(message_content);
            let payload = match payload_str {
                Ok(s) => s,
                Err(e) => {
                    error!("fail to parse mqtt message body: {}", e);
                    return;
                }
            };

            callback(payload.as_str());
        });

        let mut mqtt_client = MqttClient::start(client_options, Some(mqtt_callback))
            .expect("fail to init mqtt client");

        let topic_with_qos = mqtt_config.topics.iter().map(|s: &String| {
            (s.as_str(), QoS::Level0)
        }).collect();

        match mqtt_client.subscribe(topic_with_qos) {
            Ok(_) => info!("success subscribe {}", mqtt_config.topics.join(",")),
            Err(_) => error!("fail to subscribe topics: {}", mqtt_config.topics.join(","))
        }

        Mqtt {
            client: mqtt_client
        }
    }

}