use super::{creator::Creator, value::association::AssociationId};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociationCreator {
    pub id: AssociationId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub creators: Vec<Creator>,
}

impl AssociationCreator {
    pub fn new(id: AssociationId, name: String, creators: Vec<Creator>) -> Self {
        Self {
            id,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
            creators,
        }
    }
    pub fn from(name: String, creators: Vec<Creator>) -> Self {
        Self {
            id: Default::default(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
            creators,
        }
    }
}

pub trait AssociationCreatorRepository {
    fn find_all(&self) -> Result<Vec<AssociationCreator>>;
    // fn find_by_id(&self, association_id: AssociationId) -> Result<AssociationCreator>;
    // fn search_by_name(&self, name: String) -> Result<Vec<AssociationCreator>>;
    // fn insert(&self, association: &AssociationCreator) -> Result<AssociationCreator>;
    // fn update(&self, association: &AssociationCreator) -> Result<AssociationCreator>;
    // fn delete(&self, association_id: AssociationId) -> Result<AssociationCreator>;
}
