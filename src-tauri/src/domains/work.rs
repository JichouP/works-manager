use super::value::work::{Rate, WorkId};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: WorkId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub path: String,
    pub thumbnail: String,
    pub rate: Rate,
}

impl Work {
    pub fn new(id: WorkId, name: String, path: String, thumbnail: String, rate: Rate) -> Self {
        Self {
            id,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
            path,
            thumbnail,
            rate,
        }
    }
    pub fn from(name: String, path: String, thumbnail: String, rate: Rate) -> Self {
        Self {
            id: Default::default(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
            path,
            thumbnail,
            rate,
        }
    }
}

pub trait WorkRepository {
    fn find_all(&self) -> Result<Vec<Work>>;
    fn find_by_id(&self, work_id: WorkId) -> Result<Work>;
    fn search_by_name(&self, name: String) -> Result<Vec<Work>>;
    fn insert(&self, work: &Work) -> Result<Work>;
    fn update(&self, work: &Work) -> Result<Work>;
    fn delete(&self, work_id: WorkId) -> Result<Work>;
}
