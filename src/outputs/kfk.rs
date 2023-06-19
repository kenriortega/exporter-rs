use crate::outputs::{LogType, Output};
use kafka::client::Compression;
use std::time::Duration;

use crate::config::Cfg;
use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

pub struct Kafka;

impl Output<Kafka> {
    pub async fn send_data(&self) {
        match &self.data_received {
            LogType::LogEntryApache(data) => {
                send_data_to_kafka(self.cfg.clone(), serde_json::to_string(&data).unwrap())
            }
            LogType::LogEntryIIS(data) => {
                send_data_to_kafka(self.cfg.clone(), serde_json::to_string(&data).unwrap())
            }
            LogType::LogEntryNginx(data) => {
                send_data_to_kafka(self.cfg.clone(), serde_json::to_string(&data).unwrap())
            }
        }
    }
}

fn send_data_to_kafka(cfg: Cfg, data: String) {
    let topic_biding = cfg.kafka_opts.topics.clone().unwrap();
    let topic = topic_biding.as_str();

    let brokers: Vec<String> = vec![cfg.kafka_opts.brokers.clone().unwrap()];
    if let Err(e) = produce_message(data.as_bytes(), topic, brokers) {
        println!("Failed producing messages: {}", e);
    }
}

fn produce_message(data: &[u8], topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = Producer::from_hosts(brokers)
        // ~ give the brokers one second time to ack the message
        .with_ack_timeout(Duration::from_secs(1))
        // ~ require only one broker to ack the message
        .with_required_acks(RequiredAcks::One)
        // ~ build the producer with the above settings
        .with_compression(Compression::SNAPPY)
        .create()?;

    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we're sending 'data' as a 'value'. there will be no key
    // associated with the sent message.

    // ~ we leave the partition "unspecified" - this is a negative
    // partition - which causes the producer to find out one on its
    // own using its underlying partitioner.
    producer.send(&Record {
        topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    // producer.send(&Record::from_value(topic, data))?;

    Ok(())
}
