use super::value::{tag::TagId, work::WorkId};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTag {
    pub work_id: WorkId,
    pub tag_id: TagId,
    pub created_at: String,
    pub updated_at: String,
}

impl WorkTag {
    pub fn new(work_id: WorkId, tag_id: TagId) -> Self {
        Self {
            work_id,
            tag_id,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}

pub trait WorkRepository {
    fn find_all(&self) -> Result<Vec<WorkTag>>;
    fn find_all_by_work_id(&self, work_id: WorkId) -> Result<Vec<WorkTag>>;
    fn find_all_by_tag_id(&self, tag_id: TagId) -> Result<Vec<WorkTag>>;
    fn find_by_id(&self, work_id: WorkId, tag_id: TagId) -> Result<WorkTag>;
    fn insert(&self, work: &WorkTag) -> Result<WorkTag>;
    fn delete(&self, work_id: WorkId, tag_id: TagId) -> Result<WorkTag>;
}
