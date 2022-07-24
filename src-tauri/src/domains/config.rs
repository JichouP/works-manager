use super::value::config::ConfigId;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub id: ConfigId,
    pub created_at: String,
    pub updated_at: String,
    pub base_path: String,
}

impl Config {
    pub fn new(id: ConfigId, base_path: String) -> Self {
        Self {
            id,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            base_path,
        }
    }
}

pub trait ConfigRepository {
    fn find_all(&self) -> Result<Vec<Config>>;
    fn find_by_id(&self, config_id: ConfigId) -> Result<Config>;
    fn insert(&self, config: &Config) -> Result<Config>;
    fn update(&self, config: &Config) -> Result<Config>;
    fn delete(&self, config_id: ConfigId) -> Result<Config>;
}
