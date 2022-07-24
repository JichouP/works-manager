use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

use crate::domains::{
    association::AssociationRepository, config::ConfigRepository, creator::CreatorRepository,
};

pub struct Context {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Context {
    pub fn new(database_url: &str) -> Self {
        let pool = Self::create_pool(database_url).unwrap();
        Self { pool }
    }

    fn create_pool(database_url: &str) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
        use anyhow::Context;
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        Pool::builder()
            .build(manager)
            .context("Failed to build database pool")
    }

    pub fn association_repository(&self) -> impl AssociationRepository {
        use crate::infrastructures::repository::association::AssociationRepositoryImpl;

        AssociationRepositoryImpl {
            pool: self.pool.clone(),
        }
    }

    pub fn config_repository(&self) -> impl ConfigRepository {
        use crate::infrastructures::repository::config::ConfigRepositoryImpl;

        ConfigRepositoryImpl {
            pool: self.pool.clone(),
        }
    }

    pub fn creator_repository(&self) -> impl CreatorRepository {
        use crate::infrastructures::repository::creator::CreatorRepositoryImpl;

        CreatorRepositoryImpl {
            pool: self.pool.clone(),
        }
    }
}
