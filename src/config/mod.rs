use envy::{from_env, prefixed};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub production: bool,
    pub server_port: i16,
    pub db_connection: String,

    #[serde(skip_deserializing)]
    pub settings: Option<Settings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub test: String,
}

impl Config {
    pub fn init() -> Result<Arc<Self>, Box<std::error::Error>> {
        let mut cnfg = from_env::<Config>()?;
        let settings = prefixed("SETTINGS_").from_env::<Settings>()?;
        cnfg.settings = Some(settings);
        Ok(Arc::new(cnfg))
    }
}
