use super::value::{creator::CreatorId, work::WorkId};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCreator {
    pub work_id: WorkId,
    pub creator_id: CreatorId,
    pub created_at: String,
    pub updated_at: String,
}

impl WorkCreator {
    pub fn new(work_id: WorkId, creator_id: CreatorId) -> Self {
        Self {
            work_id,
            creator_id,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}

pub trait WorkRepository {
    fn find_all(&self) -> Result<Vec<WorkCreator>>;
    fn find_all_by_work_id(&self, work_id: WorkId) -> Result<Vec<WorkCreator>>;
    fn find_all_by_creator_id(&self, creator_id: CreatorId) -> Result<Vec<WorkCreator>>;
    fn find_by_id(&self, work_id: WorkId, creator_id: CreatorId) -> Result<WorkCreator>;
    fn insert(&self, work: &WorkCreator) -> Result<WorkCreator>;
    fn delete(&self, work_id: WorkId, creator_id: CreatorId) -> Result<WorkCreator>;
}
