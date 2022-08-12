use tracing::trace;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub categories: Vec<String>,

    pub cache: CacheConfig,
    pub web: ServerConfig,
    pub log: LogConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogConfig {
    pub level: String,
    pub dir: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheConfig {
    pub enabled: bool,
    pub update: bool,
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let fd = std::fs::File::open("config/config.yml")?;
    let cfg = serde_yaml::from_reader(fd)?;
    trace!("loaded yml config");
    return Ok(cfg)

}