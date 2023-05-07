use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;

#[derive(Serialize, Deserialize, Debug)]
struct CfgToml {
    app: Option<CfgApp>,
    sink: Option<CfgSink>,
    sources: Option<CfgSources>,
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

#[derive(Debug, Clone)]
pub enum LogType {
    Nginx,
    IIS,
    Apache,
    UnKnown,
}

impl LogType {
    pub fn from_string(sink: &str) -> Self {
        let value = match sink {
            "nginx" => LogType::Nginx,
            "iis" => LogType::IIS,
            "apache" => LogType::Apache,
            _ => LogType::UnKnown,
        };
        value
    }
}

#[derive(Debug, Clone)]
pub struct Cfg {
    pub app_name: String,
    pub logs_type: LogType,
    pub data_sources: Vec<String>,
}

impl Cfg {
    pub fn new() -> Self {
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

        Cfg {
            app_name,
            logs_type,
            data_sources,
        }
    }
}
