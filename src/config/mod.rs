pub mod sources;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::fs;
use std::io::Error as IoError;

use crate::config::sources::LogType;
use sqlx::postgres::PgPoolOptions;

#[derive(Serialize, Deserialize, Debug)]
struct CfgToml {
    app: Option<CfgApp>,
    sink: Option<CfgSink>,
    sources: Option<CfgSources>,
    kafka: Option<CfgKafka>,
    postgres: Option<CfgPostgres>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CfgApp {
    app_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CfgSink {
    logs_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CfgSources {
    data_sources: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CfgKafka {
    pub brokers: Option<String>,
    pub topics: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CfgPostgres {
    pub dsn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PostgresOpts {
    pub pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
pub struct Cfg {
    pub app_name: String,
    pub logs_type: LogType,
    pub data_sources: Vec<String>,
    pub kafka_opts: CfgKafka,
    pub pgx_opts: PostgresOpts,
}

impl Cfg {
    pub async fn new() -> Self {
        let config_filepaths: [&str; 2] = ["./settings.toml", "~/exporter-rs/settings.toml"];

        let mut content: String = "".to_owned();
        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);
            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }
        let cfg_toml: CfgToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml object");
            CfgToml {
                app: None,
                sink: None,
                sources: None,
                kafka: None,
                postgres: None,
            }
        });

        let app_name = match cfg_toml.app {
            Some(app) => app.app_name.unwrap_or_else(|| {
                println!("Missing field app_name");
                "unknown".to_owned()
            }),
            None => "unknown".to_owned(),
        };

        let logs_type = match cfg_toml.sink {
            Some(sink) => {
                let sink = sink.logs_type.unwrap_or_else(|| {
                    println!("Missing field logs_type");
                    "UnKnown".to_owned()
                });
                LogType::from_string(&sink.to_lowercase())
            }
            None => LogType::UnKnown,
        };

        let data_sources = match cfg_toml.sources {
            Some(sources) => sources.data_sources.unwrap_or_else(|| {
                println!("Missing field data_sources");
                vec![]
            }),
            None => vec![],
        };

        let kafka_opts = match cfg_toml.kafka {
            Some(opts) => {
                let brokers = opts.brokers.unwrap_or_else(|| {
                    println!("Missing field logs_type");
                    "UnKnown".to_owned()
                });
                let topics = opts.topics.unwrap_or_else(|| {
                    println!("Missing field logs_type");
                    "UnKnown".to_owned()
                });
                CfgKafka {
                    brokers: Some(brokers),
                    topics: Some(topics),
                }
            }
            _ => CfgKafka {
                brokers: None,
                topics: None,
            },
        };

        let pgx_opts = match cfg_toml.postgres {
            Some(opts) => {
                let dsn = opts.dsn.unwrap_or_else(|| {
                    println!("Missing field logs_type");
                    "UnKnown".to_owned()
                });
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&dsn)
                    .await
                    .unwrap();
                PostgresOpts { pool }
            }

            _ => {
                let pool = PgPoolOptions::default().connect("").await.unwrap();
                PostgresOpts { pool }
            }
        };

        Cfg {
            app_name,
            logs_type,
            data_sources,
            kafka_opts,
            pgx_opts,
        }
    }
}
