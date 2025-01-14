mod env;

use crate::error::Error;
use config::Config;
use serde::Deserialize;

use self::env::ENV;

const CONFIG_FILE_PATH: &str = "./config/main";
const CONFIG_FILE_PATH_PREFIX: &str = "./config/";

lazy_static! {
    /// If `AWS_SECRET_NAME` detected in runtime `ENV`, config will be
    /// parsed using AWS Secret.
    /// Otherwise, read config file.
    pub static ref C: KVConfig = {
        if !std::env::var("AWS_SECRET_NAME").unwrap_or_default().is_empty() {
            from_aws_secret().unwrap()
        } else {
            parse().unwrap()
        }
    };
}

#[derive(Clone, Deserialize, Default)]
pub struct KVConfig {
    pub db: ConfigDB,
    pub web: ConfigWeb,
    pub proof_service: ConfigProofService,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigDB {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigWeb {
    pub listen: String,
    pub port: u16,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigProofService {
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub enum ConfigCategory {
    File,
    AWSSecret,
}
impl Default for ConfigCategory {
    fn default() -> Self {
        Self::File
    }
}

/// Fetch and parse runtime ENV.
pub fn app_env() -> ENV {
    std::env::var("KV_SERVER_ENV")
        .unwrap_or_else(|_| "development".into())
        .into()
}

/// Parse config from local file or ENV.
pub fn parse() -> Result<KVConfig, Error> {
    let s = Config::builder()
        // Default
        .add_source(config::File::with_name(CONFIG_FILE_PATH).required(false))
        // app-env-based config
        .add_source(
            config::File::with_name(&format!("{}{}.toml", CONFIG_FILE_PATH_PREFIX, app_env()))
                .required(false),
        )
        // runtime-ENV-based config
        .add_source(
            config::Environment::with_prefix("KV")
                .separator("__")
                .ignore_empty(true),
        )
        .build()?;

    s.try_deserialize().map_err(|e| e.into())
}

/// `AWS_SECRET_NAME` and `AWS_SECRET_REGION` is needed.
pub fn from_aws_secret() -> Result<KVConfig, Error> {
    todo!()
}

impl KVConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db.username, self.db.password, self.db.host, self.db.port, self.db.db,
        )
    }
}
