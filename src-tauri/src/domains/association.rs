use super::value::association::AssociationId;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    pub id: AssociationId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
}

impl Association {
    pub fn new(id: AssociationId, name: String) -> Self {
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

pub trait AssociationRepository {
    fn find_all(&self) -> Result<Vec<Association>>;
    fn find_by_id(&self, association_id: AssociationId) -> Result<Association>;
    fn search_by_name(&self, name: String) -> Result<Vec<Association>>;
    fn insert(&self, association: &Association) -> Result<Association>;
    fn update(&self, association: &Association) -> Result<Association>;
    fn delete(&self, association_id: AssociationId) -> Result<Association>;
}
