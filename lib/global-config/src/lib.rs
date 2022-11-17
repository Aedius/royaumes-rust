use crate::Components::{Account, Public, Server};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Components {
    Account,
    Planet,
    Public,
    Server,
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
    hosts: HashMap<Components, Parts>,
    scripts: HashMap<Components, Vec<String>>,
    event_store: String,
    mysql: String,
    redis: String,
}

impl Config {
    pub fn load() -> Self {
        let mut hosts = HashMap::new();
        let mut scripts = HashMap::new();

        hosts.insert(
            Account,
            Parts {
                is_https: false,
                host: "127.0.0.1".to_string(),
                port: Some(8000u16),
            },
        );
        hosts.insert(
            Server,
            Parts {
                is_https: false,
                host: "127.0.0.1".to_string(),
                port: Some(8001u16),
            },
        );
        hosts.insert(
            Public,
            Parts {
                is_https: false,
                host: "127.0.0.1".to_string(),
                port: Some(3100u16),
            },
        );

        scripts.insert(Account, vec!["account.js".to_string()]);
        scripts.insert(Server, vec!["server.js".to_string()]);

        Config {
            hosts,
            scripts,
            event_store: "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
                .to_string(),
            mysql: "mysql://root:password@localhost:3306".to_string(),
            redis: "redis://localhost:6379/".to_string(),
        }
    }

    pub fn get_host(&self, component: Components) -> Option<String> {
        self.hosts.get(&component).map(|v| v.host.clone())
    }

    pub fn get_port(&self, component: Components) -> u16 {
        match self.hosts.get(&component) {
            None => 80u16,
            Some(v) => v.port.unwrap_or(80u16),
        }
    }

    pub fn get_uri(&self, component: Components) -> Option<String> {
        self.hosts.get(&component).map(|v| v.to_uri())
    }

    pub fn get_scripts(&self) -> Vec<String> {
        let mut scripts = Vec::new();

        for (component, parts) in self.hosts.clone().into_iter() {
            if let Some(list) = self.scripts.get(&component) {
                for script in list {
                    scripts.push(format!("{}/{script}", parts.to_uri()));
                }
            }
        }

        scripts
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
