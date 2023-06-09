use crate::config::Cfg;
use crate::outputs::{LogType, Output};
use chrono::prelude::*;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Streams {
    streams: Vec<StreamData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct StreamData {
    stream: HashMap<String, String>,
    values: Vec<Vec<String>>,
}

impl Streams {
    fn new(stream: HashMap<String, String>, values: Vec<Vec<String>>) -> Self {
        let mut streams = vec![];
        streams.push(StreamData { stream, values });
        Streams { streams }
    }
}

pub struct Loki;

impl Output<Loki> {
    pub async fn send_data(&self) {
        match &self.data_received {
            LogType::LogEntryApache(data) => {
                if let Err(e) = send_data_to_loki(
                    self.cfg.clone(),
                    "apache".to_string(),
                    to_string(&data).unwrap(),
                )
                .await
                {
                    println!("{}", e)
                }
            }
            LogType::LogEntryIIS(data) => {
                if let Err(e) = send_data_to_loki(
                    self.cfg.clone(),
                    "iis".to_string(),
                    to_string(&data).unwrap(),
                )
                .await
                {
                    println!("{}", e)
                }
            }
            LogType::LogEntryNginx(data) => {
                if let Err(e) = send_data_to_loki(
                    self.cfg.clone(),
                    "nginx".to_string(),
                    to_string(&data).unwrap(),
                )
                .await
                {
                    println!("{}", e)
                }
            }
        }
    }
}

async fn send_data_to_loki(
    cfg: Cfg,
    log_type: String,
    data: String,
) -> Result<Response, reqwest::Error> {
    let now = Utc::now();

    let mut labels: HashMap<String, String> = HashMap::new();
    labels.insert("log_type".to_string(), log_type);
    let values = vec![vec![format!("{}", now.timestamp_nanos()), data]];
    let streams = Streams::new(labels, values);

    let response = Client::new()
        .post(&cfg.loki_opts.url.unwrap())
        .json(&streams)
        .send()
        .await?;
    Ok(response)
}
