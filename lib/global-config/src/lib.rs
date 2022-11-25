
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
    scripts: Vec<(Parts, String)>,
    event_store: String,
    mysql: String,
    redis: String,
}

impl Config {
    pub fn load() -> Self {
        let mut hosts = Vec::new();
        let mut scripts = Vec::new();

        let public = Parts {
            is_https: false,
            host: "127.0.0.1".to_string(),
            port: Some(3100u16),
        };

        let landtish = Parts {
            is_https: false,
            host: "127.0.0.1".to_string(),
            port: Some(8001u16),
        };
        let account  = Parts {
            is_https: false,
            host: "127.0.0.1".to_string(),
            port: Some(8000u16),
        };

        hosts.push(public);
        hosts.push(landtish.clone());

        scripts.push(
            (
                account,
                "account.js".to_string(),
            ),
        );
        scripts.push(
            (
                landtish,
                "description/index.js".to_string(),
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

    pub fn get_scripts(&self) -> Vec<String> {
        self.scripts
            .iter()
            .map(|parts|format!("{}/{}", parts.0.to_uri(), parts.1) )
            .collect()
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
