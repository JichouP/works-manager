use crate::{
    domains::{
        association_creator::{AssociationCreator, AssociationCreatorRepository},
        creator::Creator,
        value::association::AssociationId,
    },
    infrastructures::database::schema::*,
};
use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use super::association::AssociationRepositoryImpl;

#[derive(Clone, Queryable, Identifiable)]
#[table_name = "associations"]
pub struct AssociationCreatorForm {
    pub id: AssociationId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub creators: Option<Creator>,
    // pub creators: Vec<Creator>,
}

impl AssociationCreatorForm {
    pub fn from(association_creator: &AssociationCreator) -> Self {
        Self {
            id: association_creator.id,
            created_at: association_creator.created_at.to_string(),
            updated_at: association_creator.updated_at.to_string(),
            name: association_creator.name.to_string(),
            creators: association_creator.creators.clone(),
        }
    }
    pub fn into(self) -> AssociationCreator {
        AssociationCreator {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name,
            creators: self.creators,
        }
    }
}

pub struct AssociationCreatorRepositoryImpl {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl AssociationCreatorRepository for AssociationRepositoryImpl {
    fn find_all(&self) -> Result<Vec<AssociationCreator>> {
        use crate::infrastructures::database::schema::associations::dsl::*;
        use crate::infrastructures::database::schema::creators::dsl::*;

        let conn = self.pool.get()?;
        let results = associations
            .left_join(creators)
            .load::<AssociationCreatorForm>(&conn)?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(results)
    }
}
