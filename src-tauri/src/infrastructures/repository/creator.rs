use crate::{
    domains::{
        association::Association,
        creator::{Creator, CreatorRepository, JoinedCreator},
        value::{association::AssociationId, creator::CreatorId},
    },
    infrastructures::{database::schema::*, repository::association::AssociationForm},
};
use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Clone, Queryable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Association)]
#[table_name = "creators"]
pub struct CreatorForm {
    pub id: CreatorId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub association_id: Option<AssociationId>,
}

impl CreatorForm {
    fn from(creator: &Creator) -> Self {
        Self {
            id: creator.id,
            created_at: creator.created_at.to_string(),
            updated_at: creator.updated_at.to_string(),
            name: creator.name.to_string(),
            association_id: creator.association_id,
        }
    }
    fn into(self) -> Creator {
        Creator {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name,
            association_id: self.association_id,
        }
    }
}

#[derive(Clone, Queryable, Identifiable)]
#[table_name = "creators"]
pub struct JoinedCreatorForm {
    pub id: CreatorId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub association: Option<Association>,
}

// impl JoinedCreatorForm {
//     fn from(creator: &Creator) -> Self {
//         Self {
//             id: creator.id,
//             created_at: creator.created_at.to_string(),
//             updated_at: creator.updated_at.to_string(),
//             name: creator.name.to_string(),
//             association: creator.association_id,
//         }
//     }
//     fn into(self) -> Creator {
//         Creator {
//             id: self.id,
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//             name: self.name,
//             association: self.association_id,
//         }
//     }
// }

#[derive(Clone, Insertable)]
#[table_name = "creators"]
pub struct NewCreatorForm {
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub association_id: Option<AssociationId>,
}

impl NewCreatorForm {
    fn from(creator: &Creator) -> Self {
        Self {
            created_at: creator.created_at.to_string(),
            updated_at: creator.updated_at.to_string(),
            name: creator.name.to_string(),
            association_id: creator.association_id,
        }
    }
}

pub struct CreatorRepositoryImpl {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl CreatorRepository for CreatorRepositoryImpl {
    fn find_all(&self) -> Result<Vec<JoinedCreator>> {
        use crate::infrastructures::database::schema::creators::dsl::*;

        let conn = self.pool.get()?;

        let result: Vec<JoinedCreator> = creators
            .left_join(associations::table)
            .load::<(CreatorForm, Option<AssociationForm>)>(&conn)?
            .into_iter()
            .map(|(c, a)| JoinedCreator {
                id: c.id,
                created_at: c.created_at,
                updated_at: c.updated_at,
                name: c.name,
                association: a.map(|a| a.into()),
            })
            .collect();

        // let result = creators
        //     .load::<CreatorForm>(&conn)?
        //     .into_iter()
        //     .map(|creator| creator.into())
        //     .collect();

        Ok(result)
    }

    fn find_by_id(&self, creator_id: CreatorId) -> Result<Creator> {
        use crate::infrastructures::database::schema::creators::dsl::*;

        let conn = self.pool.get()?;
        let creator = creators
            .filter(id.eq(creator_id))
            .first::<CreatorForm>(&conn)?
            .into();

        Ok(creator)
    }

    fn search_by_name(&self, creator_name: String) -> Result<Vec<Creator>> {
        use crate::infrastructures::database::schema::creators::dsl::*;

        let query = creators.filter(name.like(format!("%{}%", creator_name)));
        let conn = self.pool.get()?;
        let result = query
            .load::<CreatorForm>(&conn)?
            .into_iter()
            .map(|creator| creator.into())
            .collect();

        Ok(result)
    }

    fn insert(&self, creator: &Creator) -> Result<Creator> {
        use crate::infrastructures::database::schema::creators::dsl::*;

        let conn = self.pool.get()?;
        let new_creator = NewCreatorForm::from(creator);
        diesel::insert_into(creators)
            .values(&new_creator)
            .execute(&conn)?;

        // let result = self.find_by_id(creator.id)?;
        let result = creators
            .order(id.desc())
            .first::<CreatorForm>(&conn)?
            .into();

        Ok(result)
    }

    fn update(&self, creator: &Creator) -> Result<Creator> {
        use crate::infrastructures::database::schema::creators::dsl::*;

        let conn = self.pool.get()?;
        let new_creator = CreatorForm::from(creator);
        diesel::update(creators.filter(id.eq(creator.id)))
            .set(&new_creator)
            .execute(&conn)?;

        let result = self.find_by_id(creator.id)?;

        Ok(result)
    }

    fn delete(&self, creator_id: CreatorId) -> Result<Creator> {
        use crate::infrastructures::database::schema::creators::dsl::*;

        let conn = self.pool.get()?;
        diesel::delete(creators.filter(id.eq(creator_id))).execute(&conn)?;

        let result = self.find_by_id(creator_id)?;

        Ok(result)
    }
}
