use super::{
    association::Association,
    value::{association::AssociationId, creator::CreatorId},
};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    pub id: CreatorId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub association_id: Option<AssociationId>,
}

impl Creator {
    pub fn from(name: String, association_id: Option<AssociationId>) -> Self {
        Self {
            id: Default::default(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
            name,
            association_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinedCreator {
    pub id: CreatorId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub association: Option<Association>,
}

// impl JoinedCreator {
//     pub fn from(name: String, association: Option<Association>) -> Self {
//         Self {
//             id: Default::default(),
//             created_at: Utc::now().to_string(),
//             updated_at: Utc::now().to_string(),
//             name,
//             association,
//         }
//     }
// }

pub trait CreatorRepository {
    fn find_all(&self) -> Result<Vec<JoinedCreator>>;
    fn find_by_id(&self, creator_id: CreatorId) -> Result<Creator>;
    fn search_by_name(&self, name: String) -> Result<Vec<Creator>>;
    fn insert(&self, creator: &Creator) -> Result<Creator>;
    fn update(&self, creator: &Creator) -> Result<Creator>;
    fn delete(&self, creator_id: CreatorId) -> Result<Creator>;
}
