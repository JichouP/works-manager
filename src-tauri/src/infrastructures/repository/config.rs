use crate::{
    domains::{
        config::{Config, ConfigRepository},
        value::config::ConfigId,
    },
    infrastructures::database::schema::*,
};
use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Clone, Queryable, Identifiable, AsChangeset)]
#[table_name = "configs"]
pub struct ConfigForm {
    pub id: ConfigId,
    pub created_at: String,
    pub updated_at: String,
    pub base_path: String,
}

impl ConfigForm {
    fn from(config: &Config) -> Self {
        Self {
            id: config.id,
            created_at: config.created_at.to_string(),
            updated_at: config.updated_at.to_string(),
            base_path: config.base_path.to_string(),
        }
    }
    fn into(self) -> Config {
        Config {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            base_path: self.base_path,
        }
    }
}

#[derive(Clone, Insertable)]
#[table_name = "configs"]
struct NewConfigForm {
    pub created_at: String,
    pub updated_at: String,
    pub base_path: String,
}

impl NewConfigForm {
    fn from(config: &Config) -> Self {
        Self {
            created_at: config.created_at.to_string(),
            updated_at: config.updated_at.to_string(),
            base_path: config.base_path.to_string(),
        }
    }
}

pub struct ConfigRepositoryImpl {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ConfigRepository for ConfigRepositoryImpl {
    fn find_all(&self) -> Result<Vec<Config>> {
        use crate::infrastructures::database::schema::configs::dsl::*;

        let conn = self.pool.get()?;
        let result: Vec<ConfigForm> = configs.order(id.asc()).load(&conn)?;

        Ok(result.into_iter().map(|x| x.into()).collect())
    }

    fn find_by_id(&self, config_id: ConfigId) -> Result<Config> {
        use crate::infrastructures::database::schema::configs::dsl::*;

        let conn = self.pool.get()?;
        let result = configs
            .filter(id.eq(config_id))
            .first::<ConfigForm>(&conn)?
            .into();

        Ok(result)
    }

    fn insert(&self, config: &Config) -> Result<Config> {
        use crate::infrastructures::database::schema::configs::dsl::*;

        let conn = self.pool.get()?;
        let new_config = NewConfigForm::from(config);
        diesel::insert_into(configs)
            .values(&new_config)
            .execute(&conn)?;

        let result = configs.order(id.desc()).first::<ConfigForm>(&conn)?.into();

        Ok(result)
    }

    fn update(&self, config: &Config) -> Result<Config> {
        use crate::infrastructures::database::schema::configs::dsl::*;

        let conn = self.pool.get()?;
        let new_config = ConfigForm::from(config);
        diesel::update(configs.find(config.id))
            .set(&new_config)
            .execute(&conn)?;

        let result = self.find_by_id(config.id)?;

        Ok(result)
    }

    fn delete(&self, config_id: ConfigId) -> Result<Config> {
        use crate::infrastructures::database::schema::configs::dsl::*;

        let conn = self.pool.get()?;
        diesel::delete(configs.filter(id.eq(config_id))).execute(&conn)?;

        let result = self.find_by_id(config_id)?;

        Ok(result)
    }
}
