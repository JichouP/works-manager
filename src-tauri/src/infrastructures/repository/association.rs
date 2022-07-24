use crate::{
    domains::{
        association::{Association, AssociationRepository},
        value::association::AssociationId,
    },
    infrastructures::database::schema::*,
};
use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Clone, Queryable, Identifiable, AsChangeset)]
#[table_name = "associations"]
pub struct AssociationForm {
    pub id: AssociationId,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
}

impl AssociationForm {
    pub fn from(association: &Association) -> Self {
        Self {
            id: association.id,
            created_at: association.created_at.to_string(),
            updated_at: association.updated_at.to_string(),
            name: association.name.to_string(),
        }
    }
    pub fn into(self) -> Association {
        Association {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name,
        }
    }
}

#[derive(Clone, Insertable)]
#[table_name = "associations"]
pub struct NewAssociationForm {
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
}

impl NewAssociationForm {
    fn from(association: &Association) -> Self {
        Self {
            created_at: association.created_at.to_string(),
            updated_at: association.updated_at.to_string(),
            name: association.name.to_string(),
        }
    }
}

pub struct AssociationRepositoryImpl {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl AssociationRepository for AssociationRepositoryImpl {
    /// Associationを取得する
    fn find_all(&self) -> Result<Vec<Association>> {
        use crate::infrastructures::database::schema::associations::dsl::*;

        let conn = self.pool.get()?;
        let result = associations
            .order(id.asc())
            .load::<AssociationForm>(&conn)?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(result)
    }

    /// AssociationIdからAssociationを取得する
    fn find_by_id(&self, association_id: AssociationId) -> Result<Association> {
        use crate::infrastructures::database::schema::associations::dsl::*;

        let conn = self.pool.get()?;
        let result = associations
            .filter(id.eq(association_id))
            .first::<AssociationForm>(&conn)?
            .into();

        Ok(result)
    }

    /// AssociationNameからAssociationを検索する
    fn search_by_name(&self, association_name: String) -> Result<Vec<Association>> {
        use crate::infrastructures::database::schema::associations::dsl::*;

        let conn = self.pool.get()?;
        let result: Vec<AssociationForm> = associations
            .filter(name.like(format!("%{}%", association_name)))
            .load(&conn)?;

        Ok(result.into_iter().map(|x| x.into()).collect())
    }

    /// Associationを登録する
    fn insert(&self, association: &Association) -> Result<Association> {
        use crate::infrastructures::database::schema::associations::dsl::*;

        let conn = self.pool.get()?;
        let new_association = NewAssociationForm::from(association);
        diesel::insert_into(associations)
            .values(&new_association)
            .execute(&conn)?;

        let result = associations
            .order(id.desc())
            .first::<AssociationForm>(&conn)?
            .into();

        Ok(result)
    }

    /// Associationを更新する
    fn update(&self, association: &Association) -> Result<Association> {
        use crate::infrastructures::database::schema::associations::dsl::*;

        let conn = self.pool.get()?;
        let new_association = AssociationForm::from(association);
        diesel::update(associations.find(association.id))
            .set(&new_association)
            .execute(&conn)?;

        let result = self.find_by_id(association.id)?;

        Ok(result)
    }

    /// Associationを削除する
    fn delete(&self, association_id: AssociationId) -> Result<Association> {
        use crate::infrastructures::database::schema::associations::dsl::*;

        let conn = self.pool.get()?;
        let result = self.find_by_id(association_id);

        if let Ok(association) = result {
            diesel::delete(associations.find(association_id)).execute(&conn)?;

            return Ok(association);
        }

        result
    }
}
