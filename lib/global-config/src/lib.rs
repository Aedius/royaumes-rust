use crate::Components::{Private, Public};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Components {
    Public,
    Private,
}

#[derive(Clone)]
pub struct Parts {
    is_https: bool,
    host: String,
    port: Option<u16>,
}

impl Parts {
    fn to_uri(&self) -> String {
        let http = if self.is_https { "https" } else { "http" };
        let port = if self.port.is_some() {
            format!(":{}", self.port.unwrap())
        } else {
            "".to_string()
        };
        format!("{http}://{}{port}", self.host)
    }
}

pub struct Config {
    hosts: Vec<Parts>,
    scripts: HashMap<Components, (Parts, Vec<String>)>,
    event_store: String,
    mysql: String,
    redis: String,
}

impl Config {
    pub fn load() -> Self {
        let mut hosts = Vec::new();
        let mut scripts = HashMap::new();

        hosts.push(Parts {
            is_https: false,
            host: "127.0.0.1".to_string(),
            port: Some(3100u16),
        });
        hosts.push(Parts {
            is_https: false,
            host: "127.0.0.1".to_string(),
            port: Some(8001u16),
        });

        scripts.insert(
            Public,
            (
                Parts {
                    is_https: false,
                    host: "127.0.0.1".to_string(),
                    port: Some(8000u16),
                },
                vec!["account.js".to_string()],
            ),
        );
        scripts.insert(
            Public,
            (
                Parts {
                    is_https: false,
                    host: "127.0.0.1".to_string(),
                    port: Some(8001u16),
                },
                vec!["description/index.js".to_string()],
            ),
        );

        scripts.insert(
            Private,
            (
                Parts {
                    is_https: false,
                    host: "127.0.0.1".to_string(),
                    port: Some(8001u16),
                },
                vec!["stocks.js".to_string()],
            ),
        );

        Config {
            hosts,
            scripts,
            event_store: "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
                .to_string(),
            mysql: "mysql://root:password@localhost:3306".to_string(),
            redis: "redis://localhost:6379/".to_string(),
        }
    }

    pub fn get_scripts(&self, component: &Components) -> Vec<String> {
        match self.scripts.get(component) {
            None => Vec::new(),
            Some((parts, scripts)) => scripts
                .iter()
                .map(|s| format!("{}/{s}", parts.to_uri()))
                .collect(),
        }
    }

    pub fn get_hosts(&self) -> Vec<String> {
        self.hosts.iter().map(|v| v.to_uri()).collect()
    }

    pub fn event_store(&self) -> &str {
        &self.event_store
    }
    pub fn mysql(&self) -> &str {
        &self.mysql
    }
    pub fn redis(&self) -> &str {
        &self.redis
    }
}
