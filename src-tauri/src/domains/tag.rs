use super::value::tag::TagId;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
}

impl Tag {
    pub fn new(id: TagId, name: String) -> Self {
        Self {
            id,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
        }
    }
    pub fn from(name: String) -> Self {
        Self {
            id: Default::default(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
        }
    }
}

pub trait TagRepository {
    fn find_all(&self) -> Result<Vec<Tag>>;
    fn find_by_id(&self, tag_id: TagId) -> Result<Tag>;
    fn search_by_name(&self, name: String) -> Result<Vec<Tag>>;
    fn insert(&self, tag: &Tag) -> Result<Tag>;
    fn update(&self, tag: &Tag) -> Result<Tag>;
    fn delete(&self, tag_id: TagId) -> Result<Tag>;
}
