use super::config::Config;
use rusqlite::Connection;
use serde_json::{Value, to_string, from_str};
use std::io::{Error, ErrorKind};
use std::sync::Mutex;
use std::rc::Rc;


pub struct IotMessage {
    pub device_id: String,
    pub message_type: String,
    pub timestamp: i64,
    pub data: String
}

impl IotMessage {
    pub fn from_string(s: &str) -> Result<IotMessage, Error> {
        let v: Value = from_str(s)?;
        if !v["device_id"].is_string() || !v["message_type"].is_string() || !v["timestamp"].is_i64() {
            return Err(Error::new(ErrorKind::Other, "invalid json body"));
        }

        let iot_message = IotMessage {
            device_id: v["device_id"].as_str().unwrap().to_string(),
            message_type: v["message_type"].as_str().unwrap().to_string(),
            timestamp: v["timestamp"].as_i64().unwrap(),
            data: to_string(&v["data"]).unwrap()
        };

        Ok(iot_message)
    }
}

pub struct Sink {
    connection: Mutex<Connection>
}

impl Sink {
    pub fn new(config: Rc<Config>) -> Sink {
        let db_config = &config.db;
        let conn = Connection::open(&db_config.url).unwrap();

        let sink = Sink {
            connection: Mutex::new(conn)
        };

        sink.init();

        sink
    }

    fn init(&self) {
        self.connection.lock().unwrap().execute("CREATE TABLE IF NOT EXISTS iot_data(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            device TEXT NOT NULL,
            type TEXT NOT NULL,
            time INTEGER NOT NULL,
            data TEXT NOT NULL
        )", &[]).unwrap();
    }

    pub fn save(&self, msg: &IotMessage) {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare_cached("INSERT INTO iot_data (id, device, type, time, data)
            VALUES(NULL, ?, ?, ?, ?)").unwrap();
        stmt.execute(&[&msg.device_id, &msg.message_type, &msg.timestamp, &msg.data]).unwrap();
    }
}